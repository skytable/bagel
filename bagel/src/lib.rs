//! 🥯 `bagel`: Always baked, never fried
//!
//! `bagel` provides a collection of compile-time "magic" that we use frequently at
//! [Skytable](https://github.com/skytable/skytable).
//!
//! ## What bagel can do
//!
//! - [`Constdef`](derive.Constdef.html): Derive constant, compile-time default implementations
//! - [`Ctor`]: Derive constructors
//! - [`Gtor`]: Derive getters
//! - [`Stor`]: Derive setters
//! - [`def`]: Use the [default declaration syntax](https://github.com/skytable/bagel/README.md#default-declaration-syntax)
//!

#[macro_use]
mod internal_macros;
mod constdef_impls;
mod macros;

/// # Constant defaults
///
/// The [`Constdef`] trait is the heart of constant, compile-time default values. This trait
/// is automatically implemented for several types in the standard library.
///
/// ## Implementing this trait
///
/// Usually, for implementing this trait -- you'll simply need to use `#[derive(bagel::Constdef)]`
/// and the macro will do the magic for you. In other cases, you'll need to implement it yourself,
/// like this for example:
///
/// ```
/// use bagel::Constdef;
///
/// struct MyWeirdBool(bool);
///
/// impl Constdef for MyWeirdBool {
///     const DEFAULT: MyWeirdBool = MyWeirdBool(false);
/// }
///
/// ```
///
pub trait Constdef {
    /// The default value for Self
    const DEFAULT: Self;
}

// re-export macros

/// # The `Constdef` macro
/// Overcome the limits of the [`Default`] trait to get constant, compile-time default implementations.
///
/// The [`Constdef`](derive.Constdef.html) derive macro enables you to create constant,
/// compile-time [`Default`] implementations, in the simplest possible way. With restrictions
/// imposed by [RFC 911 on `const` functions](https://rust-lang.github.io/rfcs/0911-const-fn.html#detailed-design),
/// trait methods cannot currently be called in `const` contexts. To work around this, this crate
/// provides you with the [`Constdef`](derive.Constdef.html) macro that peeks into the AST and substitutes
/// the default value at compile-time. This enables you to call the `default` function in constant
/// contexts.
///
/// ## Example
/// ```
/// use bagel::Constdef;
///
/// type MyType = u16;
/// #[derive(Constdef)]
/// pub struct SpookyFriend {
///     name: String,
///     email: String,
///     friend_names: Vec<String>,
///     userid: u64,
///     my_tag: MyType,
/// }
///
/// const SPOOKY: SpookyFriend = SpookyFriend::default();
///
/// #[test]
/// fn test_struct_with_heap_fields() {
///     // spooky name; it's empty!
///     assert_eq!(SPOOKY.name, "");
///     // spooky email; it's empty!
///     assert_eq!(SPOOKY.email, "");
///     // spooky friend has no friends!
///     assert!(SPOOKY.friend_names.is_empty());
///     // spooky userid; it's 0!
///     assert_eq!(SPOOKY.userid, 0);
///     // spooky tag; it's 0!
///     assert_eq!(SPOOKY.mytag, 0);
/// }
/// ```
/// Even more complex types are supported. See [crate level docs](crate) for more information.
///
pub use dough::Constdef;

/// # Ctor: Get a constructor derived
///
/// The [`Ctor`] macro will take the fields in the order they are declared and generate a
/// constructor, that is a `YourStruct::new()` function.
///
///
/// ## Example
/// ```
/// use derived::Ctor;
///
/// #[derive(Ctor)]
/// struct MyStruct {
///     int: u32,
///     unsigned_int: i32,
/// }
///
/// let ms = MyStruct::new(1, -1);
/// assert_eq!(ms.int, 1);
/// assert_eq!(ms.unsigned_int, -1);
/// ```
///
/// # Attributes
///
/// The following attributes are available:
/// - `#[ctor_const]`: Will make your ctors constant
/// - `#[phantom]`: Will skip the specified [`PhantomData`](core::marker::PhantomData) field(s) in
/// the constructor, automatically adding `PhantomData` in the requisite positions
///
/// ## Constant constructors
///
/// To make your constructors `const`, simply add the `#[ctor_const]` attribute to the top
/// of your struct.
///
/// ### Example
///
/// ```
/// use derived::Ctor;
///
/// #[derive(Ctor)]
/// #[ctor_const]
/// pub struct MyConst {
///     a: u8,
///     b: u8,
/// }
/// // you can now use it in constant contexts
/// const MC: MyConst = MyConst::new(1, 2);
/// ```
///
pub use dough::Ctor;

/// # Gtor: Get the getters derived
///
/// Gtor takes the fields in order and generates getters for each field. For example,
/// if you have fields named `userid` and `name`, then the getters generated will be
/// `get_userid` and `get_name`, returning references to the appropriate types. In other
/// words, `get_*` named methods will be derived per your fields.
///
/// ## Example
/// ```
/// use derived::Gtor;
/// #[derive(Gtor)]
/// struct MyStruct {
///     name: String,
///     userid: u64,
/// }
///
/// let ms = MyStruct { name: "Sayan".to_owned(), userid: 16 };
/// assert_eq!(ms.get_name(), "Sayan");
/// ```
/// # Attributes
///
/// The following attributes are available:
/// - `#[gtor_const]`: Will make your gtors constant
/// - `#[gtor_skip]`: Will skip generation of getters for specific fields
/// - `#[gtor_copy]`: Makes the getter return a copy of the value, assuming that the type is [`Copy`]
/// - `#[phantom]`: Marks the field as a [`PhantomData`](core::marker::PhantomData) field, hence
///     skipping getters, setters and ctors for the field
/// - `#[gtor(...)]`: See [this example](#the-gtor-attribute)
///
/// ## The `gtor` attribute
///
/// Simply add the gtor attribute like this: `#[gtor(get, get_mut)]` on the top of your struct to
/// get mutable and immutable reference methods to the fields in your struct.
///
/// ### Example
///
/// ```
/// use derived::{Ctor, Gtor};
/// #[derive(Ctor, Gtor)]
/// #[gtor(get, get_mut)]
/// pub struct Mutable {
///     x_axis: u8,
///     y_axis: u8,
/// }
///
/// #[test]
/// fn test_get_and_get_mut() {
///     let mut m = Mutable::new(0, 0);
///     // move x by 1 unit
///     *m.get_x_axis_mut() = 1;
///     // move y by 2 units
///     *m.get_y_axis_mut() = 2;
///     assert_eq!(m.get_x_axis(), 1);
///     assert_eq!(m.get_y_axis(), 2);
/// }
/// ```
///
/// # Important notes
///
/// ## References
/// If any of the fields within the struct are primitive types that do not require large copies,
/// then the value is returned directly instead of a reference to it:
/// ```text
/// u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, str, bool, usize, isize, char, f32, f64
/// ```
///
/// ## Doc-comments
///
/// The [`Gtor`] macro will automatically add a doc comment of the form:
/// ```text
/// Returns the value for the `<struct_field>` field in struct [`<struct_name>`]
/// ```
///
pub use dough::Gtor;

/// # Stor: Get the setters derived
///
/// Stor takes the fields in order and generates setters for each field. For example,
/// if you have fields named `userid` and `name`, then the setters generated will be
/// `set_userid` and `set_name`, accepting values for the appropriate types. In other
/// words, `set_*` named methods will be derived per your fields.
///
/// ## Example
/// ```
/// use derived::Stor;
/// #[derive(Stor)]
/// struct MyStruct {
///     name: String,
///     userid: u64,
/// }
///
/// let mut ms = MyStruct { name: "Sayan".to_owned(), userid: 1 };
/// assert_eq!(ms.name, "Sayan");
/// assert_eq!(ms.userid, 1);
/// ms.set_userid(0);
/// assert_eq!(ms.userid, 0);
/// ```
///
/// # Attributes
///
/// The following attributes are available:
/// - `#[phantom]`: Skips the stor for the specified field(s), assuming they are
/// [`PhantomData`](core::marker::PhantomData) fields. This has the same effect as `#[stor_skip]`
/// but it makes it easier to use with the other macros, avoiding the need to write skips for phantom
/// fields specifically
/// - `#[stor_skip]`: Skips the stor for the specified field(s)
///
/// ## Doc-comments
///
/// The [`Stor`] macro will automatically add a doc comment of the form:
/// ```text
/// Sets the value for the `<struct_field>` field in struct [`<struct_name>`]
/// ```
///
pub use dough::Stor;
