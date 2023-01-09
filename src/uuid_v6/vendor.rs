/// Note: this file was pulle from https://github.com/jedisct1/rust-uuidv6 @ v0.1.1
/// With modifications made locally.
///
/// PR: https://github.com/jedisct1/rust-uuidv6/pull/1

use std::time::{SystemTime, UNIX_EPOCH};

fn hex_format(out: &mut [u8], bin: &[u8]) {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
    let mut j = 0;
    for b in bin {
        out[j] = HEX_CHARS[(b >> 4) as usize];
        out[j + 1] = HEX_CHARS[(b & 0x0f) as usize];
        j += 2;
    }
}

/// A 6 bytes spatially unique identifier.
#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Node {
    node_id: [u8; 6],
}

impl Node {
    /// Create a random node identifier
    pub fn new() -> Self {
        let mut node_id = [0u8; 6];
        getrandom::getrandom(&mut node_id).unwrap();
        Node { node_id }
    }

    /// Create a node identifier from a byte array
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8; 6]) -> Self {
        Node { node_id: *bytes }
    }

    /// Create a UUIDv6 base object
    #[allow(dead_code)]
    pub fn uuidv6(&self) -> UUIDv6 {
        UUIDv6::new(self)
    }
}

#[derive(Default, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct UUIDv6 {
    ts: u64,
    counter: u16,
    initial_counter: u16,
    node: Node,
}

impl UUIDv6 {
    /// Create a new UUIDv6 base object
    pub fn new(node: &Node) -> UUIDv6 {
        let ts = ((SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
            .checked_add(1221929280000000)
            .expect("Time is completely off"))
            / 100) as u64;
        let mut x = [0u8; 2];
        getrandom::getrandom(&mut x).unwrap();
        let initial_counter = u16::from_be_bytes(x);
        UUIDv6 {
            ts,
            counter: initial_counter,
            initial_counter,
            node: *node,
        }
    }

    /// Return the next bytes UUIDv6 as bytes
    pub fn create_bytes(&mut self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        let ts = self.ts;
        buf[0..8].copy_from_slice(&(ts << 4).to_be_bytes());
        let x = (0x06u16 << 12) | (((buf[6] as u16) << 8 | buf[7] as u16) >> 4);
        buf[6..8].copy_from_slice(&x.to_be_bytes());

        buf[8..10].copy_from_slice(&self.counter.to_be_bytes());
        self.counter = self.counter.wrapping_add(1);
        if self.counter == self.initial_counter {
            *self = Self::new(&self.node);
        };

        buf[10..].copy_from_slice(&self.node.node_id);

        return buf;
    }

    /// Return the next UUIDv6 string
    pub fn create(&mut self) -> String {
        let buf = self.create_bytes();
        let mut out = [0u8; 4 + 32];
        out[8] = b'-';
        out[13] = b'-';
        out[18] = b'-';
        out[23] = b'-';

        hex_format(&mut out[0..], &buf[0..4]);
        hex_format(&mut out[9..], &buf[4..6]);
        hex_format(&mut out[14..], &buf[6..8]);
        hex_format(&mut out[19..], &buf[8..10]);
        hex_format(&mut out[24..], &buf[10..]);

        String::from_utf8_lossy(&out).into_owned()
    }

}

#[derive(Default, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct UUIDv6Iterator {
    uuid: UUIDv6,
}

impl Iterator for UUIDv6Iterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.uuid.create())
    }
}

impl IntoIterator for UUIDv6 {
    type IntoIter = UUIDv6Iterator;
    type Item = String;

    fn into_iter(self) -> Self::IntoIter {
        UUIDv6Iterator { uuid: self }
    }
}

#[test]
fn test() {
    let node = Node::new();

    let mut st = node.uuidv6().into_iter();

    let uid_1 = st.next();
    let uid_2 = st.next();
    let uid_3 = st.next();
    debug_assert_ne!(uid_1, uid_2);
    debug_assert_ne!(uid_2, uid_3);
    debug_assert_ne!(uid_3, uid_1);
}
