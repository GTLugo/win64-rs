use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
  Attribute, Expr, Ident, ItemEnum, Meta, MetaList, Token, Variant, parse::Parser, parse_macro_input,
  punctuated::Punctuated, token::Comma,
};

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

fn id_attr(variant: &Variant) -> proc_macro2::TokenStream {
  if matches!(variant.ident.to_string().as_str(), "Reserved") {
    return quote! { #RESERVED_UPPER_BOUND..=#RESERVED_LOWER_BOUND };
  }

  match variant.attrs.iter().find(|a| a.path().is_ident("id")) {
    None => {
      let id = format!("WM_{}", variant.ident.to_string().to_case(convert_case::Case::UpperFlat));
      let wm = Ident::new(&id, Span::call_site());
      quote! { WindowsAndMessaging::#wm }
    }
    Some(attr) => {
      let id: Expr = attr.parse_args().unwrap();
      quote! { #id }
    }
  }
}

#[derive(Debug, Default)]
struct Params {
  w: bool,
  l: bool,
}

fn params_attr(variant: &Variant) -> Params {
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

#[proc_macro_derive(Message, attributes(id, params))]
pub fn message(input: TokenStream) -> TokenStream {
  let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
  let variants = filter_variants(variants);

  let from_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let wm = id_attr(v);
    if v.fields.is_empty() {
      quote! {
        #wm => Self::#variant_ident,
      }
    } else {
      quote! {
        #wm => Self::#variant_ident(msg),
      }
    }
  });

  let message_variants = variants.iter().map(|v| {
    let params = match params_attr(v) {
      Params { w: false, l: false } => quote! {},
      Params { w: false, l: true } => quote! { (LParam) },
      Params { w: true, l: false } => quote! { (WParam) },
      Params { w: true, l: true } => quote! { (WParam, LParam) },
    };
    let variant_ident = &v.ident;
    quote! {
      #variant_ident #params,
    }
  });

  let output = quote! {
    impl From<u32> for #ident {
      fn from(msg: u32) -> Self {
        match msg {
          #( #from_arms )*
          id => Self::Reserved(id),
        }
      }
    }

    pub enum Message {
      #( #message_variants )*
      Reserved(u32, WParam, LParam),
    }
  };

  output.into()
}

#[proc_macro_derive(Id, attributes(id, id_range))]
pub fn id(input: TokenStream) -> TokenStream {
  let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
  let variants = filter_variants(variants);

  let from_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let wm = id_attr(v);
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
    let wm = id_attr(v);
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

// #[proc_macro_derive(FromRaw)]
// pub fn from_raw_message(input: TokenStream) -> TokenStream {
//   let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
//   let variants: Vec<Variant> = variants
//     .into_iter()
//     .filter(|v| {
//       v.fields.len() == 2 // Include only regulars, not special cases
//     })
//     .collect();

//   let regular_arms = variants.iter().map(|v| {
//     let variant_ident = &v.ident;
//     quote! {
//       MessageType::#variant_ident => Self::#variant_ident(w, l),
//     }
//   });

//   let expanded = quote! {
//     impl #ident {
//       pub fn from_raw(msg: u32, w: usize, l: isize) -> Self {
//         let w = WParam(w);
//         let l = LParam(l);
//         let id = MessageType::from(msg);
//         match id {
//           #( #regular_arms )*
//           MessageType::Other => Self::Other(msg, w, l),
//           MessageType::User => Self::User(msg, w, l),
//           MessageType::App => Self::App(msg, w, l),
//           MessageType::Reserved => Self::Reserved(msg, w, l),
//           MessageType::Null => Self::Null,
//           MessageType::CancelMode => Self::CancelMode,
//           MessageType::ChildActivate => Self::ChildActivate,
//           MessageType::Close => Self::Close,
//           MessageType::Compacting => Self::Compacting(w),
//           MessageType::Create => Self::Create(l),
//           MessageType::Destroy => Self::Destroy,
//           MessageType::Enable => Self::Enable(w),
//           MessageType::EnterSizeMove => Self::EnterSizeMove,
//         }
//       }
//     }
//   };

//   // Hand the output tokens back to the compiler
//   expanded.into()
// }
