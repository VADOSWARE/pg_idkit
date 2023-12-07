# Benchmarks

## Methodology

You can test easily by running the dockerized local build of `pg_idkit` (i.e. `just build-image`), and running SQL similar to the following from `psql`:

```sql
-- Load the extension
postgres=# CREATE EXTENSION pg_idkit;
CREATE EXTENSION

-- Enable timing output in Postgres
postgres=# \timing
Timing is on.

-- Generate one million IDs w/ UUIDv6
postgres=# SELECT COUNT(idkit_uuidv6_generate()) FROM generate_series(1, 1000000);
  count
---------
 1000000
(1 row)

Time: 1053.096 ms (00:01.053)
```

## Results

This document contains benchmarks for various UUID generation strategies available in `pg_idkit`, compared with those available natively in Postgres.

| ID generation strategy       | Time (milliseconds, best of 3)    |
|------------------------------|-----------------------------------|
| `idkit_ulid_generate()`      | (/ (+ 360 364 350) 3) `358ms`     |
| `idkit_xid_generate()`       | (/ (+ 357 363 393) 3) `371ms`     |
| `idkit_timeflake_generate()` | (/ (+ 396 369 396) 3) `387ms`     |
| `idkit_uuidv7_generate()`    | (/ (+ 1060 1072 1047) 3) `1059ms` |
| `idkit_uuidv6_generate()`    | (/ (+ 1081 1082 1079) 3) `1080ms` |
| `idkit_nanoid_generate()`    | (/ (+ 1173 1192 1143) 3) `1169ms` |
| `idkit_ksuidms_generate()`   | (/ (+ 1777 1748 1778) 3) `1767ms` |
| `idkit_ksuid_generate()`     | (/ (+ 1804 1760 1910) 3) `1824ms` |
| `idkit_pushid_generate()`    | (/ (+ 2528 2548 2533) 3) `2536ms` |
| `idkit_cuid2_generate()`     | (/ (+ 4653 4580 4627) 3) `4620ms` |

`cuid` is not included because it is [deprecated in favor of `cuid2`][cuid-deprecation]

[cuid-deprecation]: https://github.com/paralleldrive/cuid#status-deprecated-due-to-security-use-cuid2-instead
