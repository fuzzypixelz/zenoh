# `PutBuilder`/`Publication` discrepancy

## Issue

Steps to reproduce:

```bash
# Should presumably be on the same network
cargo run --example z_write_delete # 1
cargo run --example z_sub_delete   # 2
```

Notice that (1) performs a PUT with `SampleKind::Delete` but (1) receives a sample with kind `SampleKind::Put`.

## Problem

[PutBuilder](https://github.com/eclipse-zenoh/zenoh/blob/master/zenoh/src/publication.rs#L165)'s impl is sound,
but [Publication](https://github.com/eclipse-zenoh/zenoh/blob/master/zenoh/src/publication.rs#L713) ain't.

## Solutions (?)

- Yeet `Publication`
- Factor out `Publication::res_sync` and `PutBuilder::res_sync`
- ???
