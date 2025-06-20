use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, Ident, ItemEnum, Token, Variant, parse_macro_input, punctuated::Punctuated};

struct Variants {
  regular: Vec<Variant>,
  fallback: Variant,
}

impl Variants {
  fn from_variants(variants: Punctuated<Variant, Token![,]>) -> Self {
    let fallback = variants
      .iter()
      .find(|v| v.attrs.iter().any(|a| a.path().is_ident("fallback")))
      .cloned()
      .unwrap();
    let normal = variants
      .into_iter()
      .filter(|v| !v.attrs.iter().any(|a| a.path().is_ident("fallback")))
      .collect();

    Self {
      regular: normal,
      fallback,
    }
  }
}

fn id_attr(variant: &Variant) -> proc_macro2::TokenStream {
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

fn from_u32(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let wm = id_attr(v);
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

fn to_id(ident: &Ident, variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      match v.fields.is_empty() {
        true => {
          let params = match params_attr(v) {
            Params { w: false, l: false } => quote! {},
            Params { w: false, l: true } => quote! { (_) },
            Params { w: true, l: false } => quote! { (_) },
            Params { w: true, l: true } => quote! { (_, _) },
          };
          quote! {
            Self::#variant_ident #params => #ident::#variant_ident,
          }
        }
        false => {
          let params = match params_attr(v) {
            Params { w: false, l: false } => quote! { (msg) },
            Params { w: false, l: true } => quote! { (msg, _) },
            Params { w: true, l: false } => quote! { (msg, _) },
            Params { w: true, l: true } => quote! { (msg, _, _) },
          };
          quote! {
            Self::#variant_ident #params => #ident::#variant_ident(*msg),
          }
        }
      }
    })
    .collect()
}

fn w(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      match v.fields.is_empty() {
        true => match params_attr(v) {
          Params { w: false, l: false } => quote! {
            Self::#variant_ident => WParam(0),
          },
          Params { w: false, l: true } => quote! {
            Self::#variant_ident(_) => WParam(0),
          },
          Params { w: true, l: false } => quote! {
            Self::#variant_ident(w) => *w,
          },
          Params { w: true, l: true } => quote! {
            Self::#variant_ident(w, _) => *w,
          },
        },
        false => match params_attr(v) {
          Params { w: false, l: false } => quote! {
            Self::#variant_ident(_) => WParam(0),
          },
          Params { w: false, l: true } => quote! {
            Self::#variant_ident(_, _) => WParam(0),
          },
          Params { w: true, l: false } => quote! {
            Self::#variant_ident(_, w) => *w,
          },
          Params { w: true, l: true } => quote! {
            Self::#variant_ident(_, w, _) => *w,
          },
        },
      }
    })
    .collect()
}

fn l(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      match v.fields.is_empty() {
        true => match params_attr(v) {
          Params { w: false, l: false } => quote! {
            Self::#variant_ident => LParam(0),
          },
          Params { w: false, l: true } => quote! {
            Self::#variant_ident(l) => *l,
          },
          Params { w: true, l: false } => quote! {
            Self::#variant_ident(_) => LParam(0),
          },
          Params { w: true, l: true } => quote! {
            Self::#variant_ident(_, l) => *l,
          },
        },
        false => match params_attr(v) {
          Params { w: false, l: false } => quote! {
            Self::#variant_ident(_) => LParam(0),
          },
          Params { w: false, l: true } => quote! {
            Self::#variant_ident(_, l) => *l,
          },
          Params { w: true, l: false } => quote! {
            Self::#variant_ident(_, _) => LParam(0),
          },
          Params { w: true, l: true } => quote! {
            Self::#variant_ident(_, _, l) => *l,
          },
        },
      }
    })
    .collect()
}

fn message_variants(variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      let params = match v.fields.is_empty() {
        true => match params_attr(v) {
          Params { w: false, l: false } => quote! {},
          Params { w: false, l: true } => quote! { (LParam) },
          Params { w: true, l: false } => quote! { (WParam) },
          Params { w: true, l: true } => quote! { (WParam, LParam) },
        },
        false => match params_attr(v) {
          Params { w: false, l: false } => quote! { (u32) },
          Params { w: false, l: true } => quote! { (u32, LParam) },
          Params { w: true, l: false } => quote! { (u32, WParam) },
          Params { w: true, l: true } => quote! { (u32, WParam, LParam) },
        },
      };
      quote! {
        #variant_ident #params,
      }
    })
    .collect()
}

fn new(ident: &Ident, variants: &Variants) -> Vec<proc_macro2::TokenStream> {
  variants
    .regular
    .iter()
    .map(|v| {
      let variant_ident = &v.ident;
      match v.fields.is_empty() {
        true => {
          let params = match params_attr(v) {
            Params { w: false, l: false } => quote! {},
            Params { w: false, l: true } => quote! { (l) },
            Params { w: true, l: false } => quote! { (w) },
            Params { w: true, l: true } => quote! { (w, l) },
          };
          quote! {
            #ident::#variant_ident => Self::#variant_ident #params,
          }
        }
        false => {
          let params = match params_attr(v) {
            Params { w: false, l: false } => quote! { (msg) },
            Params { w: false, l: true } => quote! { (msg, l) },
            Params { w: true, l: false } => quote! { (msg, w) },
            Params { w: true, l: true } => quote! { (msg, w, l) },
          };
          quote! {
            #ident::#variant_ident(msg) => Self::#variant_ident #params,
          }
        }
      }
    })
    .collect()
}

#[proc_macro_derive(Message, attributes(id, params, fallback))]
pub fn message(input: TokenStream) -> TokenStream {
  let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
  let variants = Variants::from_variants(variants);

  let fallback_ident = &variants.fallback.ident;
  let from_u32_arms = from_u32(&variants);
  let message_variants = message_variants(&variants);
  let new_arms = new(&ident, &variants);
  let to_id_arms = to_id(&ident, &variants);
  let w_arms = w(&variants);
  let l_arms = l(&variants);

  let output = quote! {
    impl From<u32> for #ident {
      fn from(msg: u32) -> Self {
        match msg {
          #( #from_u32_arms )*
          id => Self::#fallback_ident(id),
        }
      }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Message {
      #( #message_variants )*
      #fallback_ident (u32, WParam, LParam),
    }

    impl Default for Message {
      fn default() -> Self {
        Self::Null
      }
    }

    impl Message {
      pub fn new(msg: u32, w: WParam, l: LParam) -> Self {
        match #ident::from(msg) {
          #( #new_arms )*
          #ident::#fallback_ident(msg) => Self::#fallback_ident(msg, w, l),
        }
      }

      pub const fn id(&self) -> #ident {
        match self {
          #( #to_id_arms )*
          Self::#fallback_ident(msg, w, l) => #ident::#fallback_ident(*msg),
        }
      }

      pub const fn w(&self) -> WParam {
        match self {
          #( #w_arms )*
          Self::#fallback_ident(_, w, _) => *w,
        }
      }

      pub const fn l(&self) -> LParam {
        match self {
          #( #l_arms )*
          Self::#fallback_ident(_, _, l) => *l,
        }
      }
    }
  };

  output.into()
}
