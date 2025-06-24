use quote::quote;

use crate::message::attributes;

use super::variants::Variants;

pub fn from_u32(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let wm = attributes::id(v);
      match v.fields.is_empty() {
        true => quote! {
          #wm => Self::#variant_ident,
        },
        false => quote! {
          #wm => Self::#variant_ident(msg),
        },
      }
    })
    .collect()
}
