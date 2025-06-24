use quote::quote;

use crate::message::attributes;

use super::variants::Variants;

pub fn to_raw(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let wm = attributes::id(v);
      match v.fields.is_empty() {
        true => quote! {
          Self::#variant_ident => #wm,
        },
        false => quote! {
          Self::#variant_ident(id) => *id,
        },
      }
    })
    .collect()
}
