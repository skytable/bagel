#[macro_export]
/// The [`def`] macro enables you to use the [default declaration syntax](https://github.com/skytable/bagel/README.md#default-declaration-syntax).
///
/// ## Example
/// ```
/// use bagel::def;
///
/// def! {
///     pub struct MyOven {
///         starting_temperature: u8,
///         increment_temp_by: u8 = 1,
///         oven_name: &'static str = "my_kitchen_wifi_oven1",
///         items_to_bake: [&'static str; 4] = [
///             "bagels",
///             "hashbrowns",
///             "cookies",
///             "pie",
///         ],
///         people_buffer: Vec<String> = vec![
///             "one for Jamie".into(),
///             "another for Sophie".into()
///         ],
///     }
/// }
///
/// let mut myoven = MyOven::default();
///
/// assert_eq!(myoven.starting_temperature, 0);
/// assert_eq!(myoven.oven_name, "my_kitchen_wifi_oven1");
/// assert_eq!(myoven.items_to_bake[3], "pie");
/// assert_eq!(myoven.people_buffer.len(), 2);
/// ```
macro_rules! def {
    (
        $(#[$meta:meta])*
        $vis:vis struct $ident:ident {$($field:ident: $ty:ty $(= $expr:expr)?),*$(,)?}
    ) => {
        $(#[$meta])*
        $vis struct $ident {$($field: $ty,)*}
        impl ::core::default::Default for $ident {
            fn default() -> Self { Self {$($field: $crate::process_defexpr!($($expr)?),)*}}
        }
    };
}

#[macro_export]
macro_rules! process_defexpr {
    () => {
        ::core::default::Default::default()
    };
    ($expr:expr) => {
        $expr
    };
}
