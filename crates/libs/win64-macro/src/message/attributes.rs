use convert_case::Casing;
use quote::{format_ident, quote};
use syn::{Expr, Variant};

pub fn id(variant: &Variant) -> proc_macro2::TokenStream {
  match variant.attrs.iter().find(|a| a.path().is_ident("id")) {
    None => {
      let wm = format_ident!("WM_{}", variant.ident.to_string().to_case(convert_case::Case::UpperFlat));
      quote! { WindowsAndMessaging::#wm }
    }
    Some(attr) => {
      let id: Expr = attr.parse_args().unwrap();
      quote! { #id }
    }
  }
}

#[derive(Debug, Default)]
pub struct Params {
  pub w: bool,
  pub l: bool,
}

pub fn params(variant: &Variant) -> Params {
  let mut params = Params::default();

  for attr in variant.attrs.iter() {
    if attr.path().is_ident("params") {
      attr
        .parse_nested_meta(|meta| {
          if meta.path.is_ident("w") {
            params.w = true;
            return Ok(());
          }

          if meta.path.is_ident("l") {
            params.l = true;
            return Ok(());
          }

          Err(meta.error("unrecognized param"))
        })
        .unwrap();
    }
  }

  params
}
