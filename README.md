# 🥯 `bagel`: Always baked, never fried

`bagel` is a collection of macros and other _things_ that we frequently use at [Skytable](https://github.com/skytable/skytable),
primarily to get **work done at compile-time** (because we like it baked :P). This crate contains some of the stuff we use, and we're going to soon transition to entirely using this crate for the "magic".

`bagel` itself relies on `dough` which contains procedural macros which in turn rely on items from `bagel`.

## License

The `dough` and `bagel` libraries are distributed under the [Apache-2.0 License](./LICENSE).
