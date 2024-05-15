<p align="center">
  <img src="/logo.png" height="128">
</p>

[![CI](https://github.com/fjall-rs/lsm-tree/actions/workflows/test.yml/badge.svg)](https://github.com/fjall-rs/lsm-tree/actions/workflows/test.yml)
[![docs.rs](https://img.shields.io/docsrs/lsm-tree?color=green)](https://docs.rs/lsm-tree)
[![Crates.io](https://img.shields.io/crates/v/lsm-tree?color=blue)](https://crates.io/crates/lsm-tree)
![MSRV](https://img.shields.io/badge/MSRV-1.74.0-blue)

A K.I.S.S. implementation of log-structured merge trees (LSM-trees/LSMTs) in Rust.

> This crate only provides a primitive LSM-tree, not a full storage engine.
> For example, it does not ship with a write-ahead log.
> You probably want to use https://github.com/fjall-rs/fjall instead.

```bash
cargo add lsm-tree
```

## About

This is the most feature-rich LSM-tree implementation in Rust! It features:

- Thread-safe BTreeMap-like API
- 100% safe & stable Rust
- Block-based tables with LZ4 compression
- Range & prefix searching with forward and reverse iteration
- Size-tiered, (concurrent) Levelled and FIFO compaction 
- Multi-threaded flushing (immutable/sealed memtables)
- Partitioned block index to reduce memory footprint and keep startup time short [[1]](#footnotes)
- Block caching to keep hot data in memory
- Bloom filters to increase point lookup performance (`bloom` feature, disabled by default)
- Snapshots (MVCC)

Keys are limited to 65536 bytes, values are limited to 2^32 bytes. As is normal with any kind of storage
engine, larger keys and values have a bigger performance impact.

## Feature flags

#### bloom

Uses bloom filters to reduce disk I/O for non-existing keys. Improves point read performance, but increases memory usage.

*Disabled by default.*

## Stable disk format

The disk format is stable as of 1.0.0. Future breaking changes will result in a major version bump and a migration path.

## License

All source code is licensed under MIT OR Apache-2.0.

All contributions are to be licensed as MIT OR Apache-2.0.

## Development

### Run benchmarks

```bash
cargo bench --features bloom
```

## Footnotes

[1] https://rocksdb.org/blog/2017/05/12/partitioned-index-filter.html
