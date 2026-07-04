/// # microkernel-macros
///
/// Procedural macros for the microkernel plugin system.
///
/// ## `#[derive(Plugin)]`
///
/// Automatically implements the boilerplate for a `Plugin<E>` implementation:
/// - Validates that the struct has an `env: E` field.
/// - Generates a default `name()` implementation returning the struct's type name.
///
/// ### Usage
/// ```rust,ignore
/// use microkernel_macros::Plugin;
///
/// #[derive(Plugin)]
/// pub struct MyPlugin<E: SystemEnv> {
///     env: E,
/// }
/// ```
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Plugin)]
pub fn derive_plugin(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();

    // Extract generic parameters to reconstruct impl<E: SystemEnv>
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate a `name()` method implementation
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Returns the plugin's type name for use in log messages and diagnostics.
            pub fn plugin_name() -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(expanded)
}
