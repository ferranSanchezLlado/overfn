//! # Overload functions
//!
//! This crate allows you to overload functions with the same name but with different number of arguments through the
//! [`overload`](macro@overload) macro. After overloading all the functions, you need to use the [`macros!()`](macro@macros)
//! to genarate the macros to invoke the overloaded functions.
//!
//! # Example
//!
//! ```rust
//! use overfn::*;
//!
//! #[overload]
//! fn test(item: usize) -> usize {
//!     item
//! }
//!
//! #[overload]
//! fn test(left: usize, right: usize) -> usize {
//!     left + right
//! }
//!
//! struct Test(usize);
//!
//! impl Test {
//!     #[overload(Test)]
//!     fn new() -> Self {
//!         Self(0)
//!     }
//!
//!     #[overload(Test)]
//!     fn new(item: usize) -> Self {
//!         Self(item)
//!     }
//!
//!     #[overload(Test)]
//!     fn test(&self) -> usize {
//!         self.0
//!     }
//!
//!     #[overload(Test)]
//!     fn test(&self, other: usize) -> usize {
//!         self.0 + other
//!     }
//! }
//!
//! macros!();
//!
//! assert_eq!(test!(2), 2);
//! assert_eq!(test!(2, 2), 4);
//!
//! let test = Test_new!();
//! assert_eq!(test.0, 0);
//!
//! let test = Test_new!(2);
//! assert_eq!(test.0, 2);
//!
//! assert_eq!(Test_test!(test), 2);
//! assert_eq!(Test_test!(test, 2), 4);
//! ```
//!
//! # Limitations
//!
//! - Curretly, you can't overload a function with the same number of arguments with different types.
//! - You need to use the [`macros!()`](macro@macros) macro to generate the macros to call the overloaded functions.
//! - If you overload a class method or instance method, you need to pass the class name in the attribute.
use proc_macro::TokenStream;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use once_cell::sync::Lazy;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, ItemFn};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ArgType {
    Struct(String),
    Instance,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FunctionData {
    name: String,
    n_args: usize,
    arg_type: ArgType,
}

impl FunctionData {
    fn new(name: String, arg: ArgType, function: &ItemFn) -> Self {
        Self {
            name,
            n_args: function.sig.inputs.len(),
            arg_type: arg,
        }
    }
}
static FUNCTIONS: Lazy<Mutex<HashMap<String, HashSet<FunctionData>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Overload a function with a new function with the same name but with different number of arguments.
///
/// After overloading a function, you need to use the [`macros!()`](macro@macros) macro to generate the macros to call the
/// overloaded functions.
///
/// # Example
///
/// ```rust
/// use overfn::*;
///
/// #[overload]
/// fn test(item: usize) -> usize {
///     item
/// }
///
/// #[overload]
/// fn test(left: usize, right: usize) -> usize {
///     left + right
/// }
///
/// struct Test(usize);
///
/// impl Test {
///     #[overload(Test)]
///     fn new() -> Self {
///         Self(0)
///     }
///
///     #[overload(Test)]
///     fn new(item: usize) -> Self {
///         Self(item)
///     }
///
///     #[overload(Test)]
///     fn test(&self) -> usize {
///         self.0
///     }
///
///     #[overload(Test)]
///     fn test(&self, other: usize) -> usize {
///         self.0 + other
///     }
/// }
///
/// macros!();
///
/// assert_eq!(test!(2), 2);
/// assert_eq!(test!(2, 2), 4);
///
/// let test = Test_new!();
/// assert_eq!(test.0, 0);
///
/// let test = Test_new!(2);
/// assert_eq!(test.0, 2);
///
/// assert_eq!(Test_test!(test), 2);
/// assert_eq!(Test_test!(test, 2), 4);
/// ```
///
/// # Limitations
///
/// - Curretly, you can't overload a function with the same number of arguments with different types.
/// - You need to use the [`macros!()`](macro@macros) macro to generate the macros to call the overloaded functions.
/// - If you overload a class method or instance method, you need to pass the class name in the attribute.
#[proc_macro_attribute]
pub fn overload(attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(function as ItemFn);

    let ident = &function.sig.ident;
    let n_args = &function.sig.inputs.len();
    let new_ident = format_ident!("{}_{}", ident, n_args, span = ident.span());

    let (arg_type, macro_ident) = match attr.is_empty() {
        true => (ArgType::Other, ident.to_string()),
        false => {
            let struct_name = parse_macro_input!(attr as Ident);
            let arg_type = match function.sig.inputs.first() {
                Some(arg) if matches!(arg, syn::FnArg::Receiver(_)) => ArgType::Instance,
                _ => ArgType::Struct(struct_name.to_string()),
            };
            (arg_type, format!("{}_{}", struct_name, ident))
        }
    };

    let new = FUNCTIONS
        .lock()
        .unwrap()
        .entry(macro_ident)
        .or_insert_with(Default::default)
        .insert(FunctionData::new(
            new_ident.to_string(),
            arg_type,
            &function,
        ));

    if !new {
        panic!(
            "Function {} with {} arguments already exists",
            ident, n_args
        );
    }

    function.sig.ident = new_ident;

    quote! { #function }.into()
}

/// Generate the macros to call the overloaded functions. You need to call this macro after all the functions are
/// overloaded.
///
/// For more information, see the [`overload`](macro@overload) macro.
///
/// # Example
///
/// ```rust
/// use overfn::*;
///
/// #[overload]
/// fn add(item: usize) -> usize {
///    10 + item
/// }
///
/// #[overload]
/// fn add(left: usize, right: usize) -> usize {
///   left + right
/// }
///
/// macros!();
///
/// assert_eq!(add!(2), 12);
/// assert_eq!(add!(2, 2), 4);
/// ```
#[proc_macro]
pub fn macros(_item: TokenStream) -> TokenStream {
    let macros = FUNCTIONS
        .lock()
        .unwrap()
        .iter()
        .map(|(name, functions)| {
            let options = functions
                .iter()
                .map(|data| {
                    let func = format_ident!("{}", data.name);
                    let mut func_args = (0..data.n_args)
                        .map(|i| format_ident!("arg_{}", i))
                        .map(|arg| (quote! { $ #arg }))
                        .collect::<Vec<_>>();

                    let input_args = func_args
                        .iter()
                        .map(|arg| quote! { #arg: expr })
                        .collect::<Vec<_>>();

                    let pre_args = match &data.arg_type {
                        ArgType::Struct(name) => {
                            let name = format_ident!("{}", name);
                            quote! { #name:: }
                        }
                        ArgType::Instance => {
                            let self_arg = func_args.remove(0);
                            quote! { #self_arg. }
                        }
                        ArgType::Other => quote! {},
                    };

                    quote! {
                        (#(#input_args),*) => (
                            #pre_args #func(#(#func_args),*)
                        )
                    }
                })
                .collect::<Vec<_>>();
            let name = format_ident!("{}", name);
            quote! {
                macro_rules! #name {
                    #(#options);*
                }
            }
        })
        .map(TokenStream::from)
        .collect::<TokenStream>();
    FUNCTIONS.lock().unwrap().clear();
    macros
}
