# Dynamization of static containers

[![crate](https://img.shields.io/crates/v/dynamization)](https://crates.io/crates/dynamization/)
[![docs](https://docs.rs/dynamization/badge.svg)](https://docs.rs/dynamization/)

A crate allowing one to endow a static (i.e. not supporting insertion) 
data structure with an effective insertion procedure with 
a small decrease in query performance.

## Usage

Simply include 

```
dynamization = "0.3"
```

in your `Cargo.toml`.

## Examples

This part of readme is __WIP__. You can read the [docs](https://docs.rs/dynamization/).

## Versions

* `0.4.0`: WIP
* `0.3.0`: Updated/fixed docs & added two new dynamization variants & `SVQueue` has now a `Strategy` generic parameter.
* `0.2.0`: Bugfixes && some renames && better docs.
* `0.1.0`: Initial commit (yanked: the provided `SortedVec` was unsound).

