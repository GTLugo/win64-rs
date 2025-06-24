use quote::{format_ident, quote};
use syn::Ident;

use crate::message::attributes::{self, Params};

use super::variants::Variants;

pub fn id(ident: &Ident, variants: &Variants) -> Vec<proc_macro2::TokenStream> {
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
            _ => quote! { (#substruct_name { .. }) },
          };
          quote! {
            Self::#variant_ident #params => #ident::#variant_ident,
          }
        }
        false => {
          let params = quote! { (#substruct_name { id, .. }) };
          quote! {
            Self::#variant_ident #params => #ident::#variant_ident(*id),
          }
        }
      }
    })
    .collect()
}

pub fn w(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let substruct_name = format_ident!("{}Message", variant_ident);
      match v.fields.is_empty() {
        true => match attributes::params(v) {
          Params { w: false, l: false } => quote! {
            Self::#variant_ident => WParam(0),
          },
          Params { w: false, l: _ } => quote! {
            Self::#variant_ident(#substruct_name { .. }) => WParam(0),
          },
          Params { w: true, l: _ } => quote! {
            Self::#variant_ident(#substruct_name { w, .. }) => *w,
          },
        },
        false => match attributes::params(v) {
          Params { w: false, l: _ } => quote! {
            Self::#variant_ident(#substruct_name { .. }) => WParam(0),
          },
          Params { w: true, l: _ } => quote! {
            Self::#variant_ident(#substruct_name { w, .. }) => *w,
          },
        },
      }
    })
    .collect()
}

pub fn l(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let substruct_name = format_ident!("{}Message", variant_ident);
      match v.fields.is_empty() {
        true => match attributes::params(v) {
          Params { w: false, l: false } => quote! {
            Self::#variant_ident => LParam(0),
          },
          Params { w: _, l: false } => quote! {
            Self::#variant_ident(#substruct_name { .. }) => LParam(0),
          },
          Params { w: _, l: true } => quote! {
            Self::#variant_ident(#substruct_name { l, .. }) => *l,
          },
        },
        false => match attributes::params(v) {
          Params { w: _, l: false } => quote! {
            Self::#variant_ident(#substruct_name { .. }) => LParam(0),
          },
          Params { w: _, l: true } => quote! {
            Self::#variant_ident(#substruct_name { l, .. }) => *l,
          },
        },
      }
    })
    .collect()
}
