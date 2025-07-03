use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemEnum, LitStr, parse_macro_input};

pub fn macro_impl(input: TokenStream) -> TokenStream {
  let ItemEnum { variants, .. } = parse_macro_input!(input as ItemEnum);

  let system_variants: Vec<proc_macro2::TokenStream> = variants
    .into_iter()
    .filter(|v| &v.ident.to_string() != "Custom")
    .map(|v| {
      let variant = v.ident;
      let constant = format_ident!("{}", variant.to_string().to_case(Case::UpperSnake));
      let string = LitStr::new(&variant.to_string(), variant.span());
      quote! {
        Self::#variant => { static #constant: ClassName = ClassName(windows_sys::w!(#string)); #constant.0 },
      }
    })
    .collect();

  let output = quote! {
    impl WindowClass {
      pub const fn atom(&self) -> *const u16 {
        struct ClassName(*const u16);
        unsafe impl Sync for ClassName {}

        match self {
          WindowClass::Custom(class) => class.atom(),
          #( #system_variants )*
        }
      }
    }
  };

  output.into()
}
