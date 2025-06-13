use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, Ident, ItemEnum, Variant, parse_macro_input, punctuated::Punctuated, token::Comma};

fn filter_variants(variants: Punctuated<Variant, Comma>) -> Vec<Variant> {
  variants
    .into_iter()
    .filter(|v| {
      !matches!(v.ident.to_string().as_str(), "Reserved") // Skip this one as it is a special case
    })
    .collect()
}

const RESERVED_UPPER_BOUND: u32 = 0x0000;
const RESERVED_LOWER_BOUND: u32 = 0x03FF;

fn id_range_expr(variant: &Variant) -> proc_macro2::TokenStream {
  if matches!(variant.ident.to_string().as_str(), "Reserved") {
    return quote! { #RESERVED_UPPER_BOUND..=#RESERVED_LOWER_BOUND };
  }

  // let wm = Ident::new(&id, Span::call_site());
  if let Some(a) = variant.attrs.iter().find(|a| {
    let ident = a.path().get_ident().unwrap();
    a.path().is_ident(&Ident::new("id", ident.span()))
  }) {
    let attr: Expr = a.parse_args().unwrap();
    return quote! { #attr..=#attr };
  }

  match variant.attrs.iter().find(|a| {
    let ident = a.path().get_ident().unwrap();
    a.path().is_ident(&Ident::new("id_range", ident.span()))
  }) {
    None => {
      let id = format!("WM_{}", variant.ident.to_string().to_case(convert_case::Case::UpperFlat));
      let wm = Ident::new(&id, Span::call_site());
      quote! { WindowsAndMessaging::#wm..=WindowsAndMessaging::#wm }
    }
    Some(a) => {
      let attr: Expr = a.parse_args().unwrap();
      quote! { #attr }
    }
  }
}

#[proc_macro_derive(Id, attributes(id, id_range))]
pub fn message_id(input: TokenStream) -> TokenStream {
  let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
  let variants = filter_variants(variants);

  let from_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let wm = id_range_expr(v);
    // if v.fields.is_empty() {
    quote! {
      #wm => Self::#variant_ident,
    }
    // } else {
    //   quote! {
    //     #wm => Self::#variant_ident(msg),
    //   }
    // }
  });

  let to_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let wm = id_range_expr(v);
    // if v.fields.is_empty() {
    quote! {
      Self::#variant_ident => #wm,
    }
    // } else {
    //   quote! {
    //     Self::#variant_ident(id) => id,
    //   }
    // }
  });

  let expanded = quote! {
    impl From<u32> for #ident {
      fn from(msg: u32) -> Self {
        match msg {
          #( #from_arms )*
          id => Self::Reserved,
        }
      }
    }

    impl #ident {
      pub const fn to_id_range(self) -> std::ops::RangeInclusive<u32> {
        match self {
          #( #to_arms )*
          Self::Reserved => #RESERVED_UPPER_BOUND..=#RESERVED_LOWER_BOUND,
        }
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}

#[proc_macro_derive(FromRaw)]
pub fn from_raw_message(input: TokenStream) -> TokenStream {
  let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
  let variants: Vec<Variant> = variants
    .into_iter()
    .filter(|v| {
      !matches!(v.ident.to_string().as_str(), "Reserved" | "Other" | "User" | "App") // Skip ranges
    })
    .collect();

  let regular_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    quote! {
      MessageType::#variant_ident => Self::#variant_ident(w, l),
    }
  });

  let expanded = quote! {
    impl #ident {
      pub fn from_raw(msg: u32, w: usize, l: isize) -> Self {
        let w = WParam(w);
        let l = LParam(l);
        let id = MessageType::from(msg);
        match id {
          #( #regular_arms )*
          MessageType::Other => Self::Other(msg, w, l),
          MessageType::User => Self::User(msg, w, l),
          MessageType::App => Self::App(msg, w, l),
          MessageType::Reserved => Self::Reserved(msg, w, l),
        }
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}
