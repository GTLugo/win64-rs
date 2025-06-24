use quote::{format_ident, quote};
use syn::{Token, Variant, punctuated::Punctuated};

use crate::message::attributes::{self, Params};

pub struct Variants {
  pub regular: Vec<Variant>,
  pub fallback: Variant,
}

impl Variants {
  pub fn from_variants(variants: Punctuated<Variant, Token![,]>) -> Self {
    let fallback = variants
      .iter()
      .find(|v| v.attrs.iter().any(|a| a.path().is_ident("fallback")))
      .cloned()
      .unwrap();
    let regular = variants
      .into_iter()
      .filter(|v| !v.attrs.iter().any(|a| a.path().is_ident("fallback")))
      .collect();

    Self { regular, fallback }
  }
}

pub fn message_variants(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let substruct_name = format_ident!("{}Message", variant_ident);
      let params = match v.fields.is_empty() {
        true => match attributes::params(v) {
          Params { w: false, l: false } => quote! {},
          _ => quote! { (#substruct_name) },
        },
        false => quote! { (#substruct_name) },
      };
      quote! {
        #variant_ident #params,
      }
    })
    .collect()
}

pub fn submessage_structs(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let struct_name = format_ident!("{}Message", variant_ident);
      let derives = quote! { #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] };
      match v.fields.is_empty() {
        true => match attributes::params(v) {
          Params { w: false, l: false } => quote! {},
          Params { w: false, l: true } => quote! {
            #derives
            pub struct #struct_name { pub l: LParam }
          },
          Params { w: true, l: false } => quote! {
            #derives
            pub struct #struct_name { pub w: WParam }
          },
          Params { w: true, l: true } => quote! {
            #derives
            pub struct #struct_name { pub w: WParam, pub l: LParam }
          },
        },
        false => match attributes::params(v) {
          Params { w: false, l: false } => quote! {
            #derives
            pub struct #struct_name { pub id: u32 }
          },
          Params { w: false, l: true } => quote! {
            #derives
            pub struct #struct_name { pub id: u32, pub l: LParam }
          },
          Params { w: true, l: false } => quote! {
            #derives
            pub struct #struct_name { pub id: u32, pub w: WParam }
          },
          Params { w: true, l: true } => quote! {
            #derives
            pub struct #struct_name { pub id: u32, pub w: WParam, pub l: LParam }
          },
        },
      }
    })
    .collect()
}
