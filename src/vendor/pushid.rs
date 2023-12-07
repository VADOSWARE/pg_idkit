extern crate rand;

use rand::Rng;

use std::time::{SystemTime, UNIX_EPOCH};

const PUSH_CHARS: &str = "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz";

pub trait PushIdGen {
    fn get_id(&mut self) -> String;
}

pub struct PushId {
    /// Seconds since the UNIX epoch
    last_time: u64,
    previous_indices: [usize; 12],
}

impl PushId {
    pub fn new() -> Self {
        let random_indices = PushId::generate_random_indices();
        PushId {
            last_time: 0,
            previous_indices: random_indices,
        }
    }

    fn gen_random_indices(&self, is_duplicate_time: bool) -> [usize; 12] {
        if is_duplicate_time {
            // If the timestamp hasn't changed since last push, use the same random number, except incremented by 1.
            let mut indices_copy = self.previous_indices.clone();

            for x in (0..12).rev() {
                if indices_copy[x] == 63 {
                    indices_copy[x] = 0;
                } else {
                    indices_copy[x] = indices_copy[x] + 1;
                    break;
                }
            }
            indices_copy
        } else {
            PushId::generate_random_indices()
        }
    }

    fn generate_random_indices() -> [usize; 12] {
        let mut rng = rand::thread_rng();
        let mut random_indices = [0; 12];
        for i in 0..12 {
            let n = rng.gen::<f64>() * 64 as f64;
            random_indices[i] = n as usize;
        }
        random_indices
    }

    fn gen_time_based_prefix(now: u64, mut acc: [usize; 8], i: u8) -> [usize; 8] {
        let index = (now % 64) as usize;
        acc[i as usize] = index;

        match now / 64 {
            new_now if new_now > 0 => PushId::gen_time_based_prefix(new_now, acc, i - 1),
            _ => acc, // We've reached the end of "time". Return the indices
        }
    }

    fn indices_to_characters(indices: Vec<&usize>) -> String {
        indices.iter().fold(String::from(""), |acc, &&x| {
            acc + &PUSH_CHARS
                .chars()
                .nth(x)
                .expect("Index out of range")
                .to_string()
        })
    }

    /// Retrieve the number of milliseconds since
    fn get_now() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected time seed, EPOCH is not in the past");
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
    }

    /// Get the milliseconds since UNIX epoch for the PushId
    pub fn last_time_millis(&self) -> u64 {
        self.last_time.into()
    }
}

impl PushIdGen for PushId {
    fn get_id(&mut self) -> String {
        let now = PushId::get_now();
        let is_duplicate_time = now == self.last_time;
        let prefix = PushId::gen_time_based_prefix(now, [0; 8], 7);
        let suffix = PushId::gen_random_indices(self, is_duplicate_time);
        self.previous_indices = suffix;
        self.last_time = PushId::get_now();
        let all = prefix.iter().chain(suffix.iter()).collect::<Vec<&usize>>();
        PushId::indices_to_characters(all)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{PushId, PushIdGen};

    /// Ensure that timestamps work properly
    #[test]
    fn test_timestamp() {
        let mut pushid = PushId::new();
        let id = pushid.get_id();
        assert!(!id.is_empty(), "generated pushid");

        let now = SystemTime::now();
        let millis_since = now
            .duration_since(UNIX_EPOCH)
            .expect("invalid epoch")
            .as_millis();
        let millis_since_pushid = pushid.last_time_millis() as u128;
        assert!(
            millis_since - millis_since_pushid < 10,
            "retrieved pushid generation time was within 10ms from now()"
        );
    }
}
