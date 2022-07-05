# 🥯 `bagel`: Always baked, never fried

![Rust stable](https://img.shields.io/badge/rust-stable-brightgreen?style=for-the-badge) [![docs.rs](https://img.shields.io/docsrs/bagel?style=for-the-badge)](https://docs.rs/bagel) [![Crates.io](https://img.shields.io/crates/v/bagel?style=for-the-badge)](https://crates.io/crates/bagel) [![Discord](https://img.shields.io/discord/729378001023926282?style=for-the-badge)](https://discord.gg/QptWFdx) [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/skytable/bagel/Rust?style=for-the-badge)](https://github.com/skytable/bagel/actions)

`bagel` is a collection of macros and other _things_ that we frequently use at [Skytable](https://github.com/skytable/skytable),
primarily to get **work done at compile-time** (because we like it baked :P). This crate contains some of the stuff we use, and we'll add more of the "magic" soon.

## Importing

```toml
bagel = "0.1"
```

## What bagel can do

- `def`: Use the [default declaration syntax](#default-declaration-syntax)
- `Ctor`: Derive constructors:
  - Full lifetimes, generics and where clause support
  - `#[phantom]`: Auto elide `PhantomData` fields
  - `#[ctor_const]`: Make the constructor a `const fn`
- `Gtor`: Derive getters:
  - Full lifetimes, generics and where clause support
  - Advanced attributes: `#[gtor_const]`, `#[gtor_copy]`, `#[gtor_skip]`, `#[phantom]` and `#[gtor]`
- `Stor`: Derive setters
  - Full lifetimes, generics and where clause support
  - Skip setter with `#[stor_skip]` or `#[phantom]`
- `Constdef`: Derive constant, compile-time default implementations. See [an example here](#constdef-example)

## Default declaration syntax

The _default declaration syntax_ is an alternative way to implement defaults for your structs (and enums
soon). It looks like this:

1. Use the default trait:
   ```
   field: type
   ```
2. Use your specified expression:
   ```
   field: type = expression
   ```

Here's an example:

```rust
use bagel::def;

def! {
    #[derive(Debug)]
    pub struct MyOven {
        starting_temperature: u8,
        increment_temp_by: u8 = 1,
        oven_name: &'static str = "my_kitchen_wifi_oven1",
        items_to_bake: [&'static str; 4] = [
            "bagels",
            "hashbrowns",
            "cookies",
            "pie",
        ],
        people_buffer: Vec<String> = vec![
            "one for Jamie".into(),
            "another for Sophie".into()
        ],
    }
}

let mut myoven = MyOven::default();

assert_eq!(myoven.starting_temperature, 0);
assert_eq!(myoven.oven_name, "my_kitchen_wifi_oven1");
assert_eq!(myoven.items_to_bake[3], "pie");
assert_eq!(myoven.people_buffer.len(), 2);
```

## `Constdef` example

```rust
use bagel::Constdef;

#[derive(Constdef)]
struct Port {
    requests: usize,
    admin: bool,
}

#[derive(Constdef)]
struct PortLogger {
    ports: [Port; 65536],
    root_pid: usize,
}

const PORT_LOGGER: PortLogger = PortLogger::default();

assert_eq!(PORT_LOGGER.ports[0].requests, 0);
assert_eq!(PORT_LOGGER.ports[65535].admin, false);
```

## License

The `dough` and `bagel` libraries are distributed under the [Apache-2.0 License](./LICENSE).
