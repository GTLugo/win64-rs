use quote::{format_ident, quote};
use syn::Ident;

use crate::message::attributes::{self, Params};

use super::variants::Variants;

pub fn new(ident: &Ident, variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let substruct_name = format_ident!("{}Message", variant_ident);
      match v.fields.is_empty() {
        true => {
          let params = match attributes::params(v) {
            Params { w: false, l: false } => quote! {},
            Params { w: false, l: true } => quote! { (#substruct_name { l }) },
            Params { w: true, l: false } => quote! { (#substruct_name { w }) },
            Params { w: true, l: true } => quote! { (#substruct_name { w, l }) },
          };
          quote! {
            #ident::#variant_ident => Self::#variant_ident #params,
          }
        }
        false => {
          let params = match attributes::params(v) {
            Params { w: false, l: false } => quote! { (#substruct_name { id }) },
            Params { w: false, l: true } => quote! { (#substruct_name { id, l }) },
            Params { w: true, l: false } => quote! { (#substruct_name { id, w }) },
            Params { w: true, l: true } => quote! { (#substruct_name { id, w, l }) },
          };
          quote! {
            #ident::#variant_ident(id) => Self::#variant_ident #params,
          }
        }
      }
    })
    .collect()
}
