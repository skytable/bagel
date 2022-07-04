#[macro_use]
mod macros;
mod constdef_impls;

/// # Constant defaults
///
/// The [`Constdef`] trait is the heart of constant, compile-time default values. This trait
/// is automatically implemented for several types in the standard library.
///
/// ## Implementing this trait
///
/// Usually, for implementing this trait -- you'll simply need to use `#[derive(constant::Constdef)]`
/// and the macro will do the magic for you. In other cases, you'll need to implement it yourself,
/// like this for example:
///
/// ```
/// use constant::Constdef;
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
/// use constant::Constdef;
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
