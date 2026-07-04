/// # microkernel-macros
///
/// 微内核插件系统的过程宏。
///
/// ## `#[derive(Plugin)]`
///
/// 自动为 `Plugin<E>` 实现样板代码：
/// - 验证结构体是否有 `env: E` 字段。
/// - 生成默认的 `name()` 实现，返回结构体的类型名。
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

    // 提取泛型参数以重构 impl<E: SystemEnv>
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 生成 `name()` 方法实现
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// 返回插件的类型名，用于日志消息和诊断。
            pub fn plugin_name() -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(expanded)
}
