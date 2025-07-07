mod accessors;
mod attributes;
mod from_u32;
mod new;
mod to_raw;
mod variants;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemEnum, parse_macro_input};

use self::variants::Variants;

pub fn macro_impl(input: TokenStream) -> TokenStream {
  let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);
  let variants = Variants::from_variants(variants);

  let fallback_ident = &variants.fallback.ident;
  let fallback_substruct_name = format_ident!("{}Message", fallback_ident);
  let to_raw_arms = to_raw::to_raw(&variants);
  let from_u32_arms = from_u32::from_u32(&variants);

  let message_variants = variants::message_variants(&variants);
  let submessages = variants::submessage_structs(&variants);
  let new_arms = new::new(&ident, &variants);
  let id_arms = accessors::id(&ident, &variants);
  let w_arms = accessors::w(&variants);
  let l_arms = accessors::l(&variants);

  let output = quote! {
    impl #ident {
      pub const fn to_raw(&self) -> u32 {
        match self {
          #( #to_raw_arms )*
          Self::#fallback_ident(id) => *id,
        }
      }
    }

    impl From<u32> for #ident {
      fn from(msg: u32) -> Self {
        match msg {
          #( #from_u32_arms )*
          id => Self::#fallback_ident(id),
        }
      }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Message {
      #( #message_variants )*
      #fallback_ident(#fallback_substruct_name),
    }

    #(#submessages)*
    #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct #fallback_substruct_name { pub id: u32, pub w: WParam, pub l: LParam }

    impl Default for Message {
      fn default() -> Self {
        Self::Null
      }
    }

    impl Message {
      pub fn new(id: #ident, w: WParam, l: LParam) -> Self {
        match id {
          #( #new_arms )*
          #ident::#fallback_ident(id) => Self::#fallback_ident(#fallback_substruct_name { id, w, l }),
        }
      }

      pub const fn id(&self) -> #ident {
        match self {
          #( #id_arms )*
          Self::#fallback_ident(#fallback_substruct_name { id, .. }) => #ident::#fallback_ident(*id),
        }
      }

      pub const fn w(&self) -> WParam {
        match self {
          #( #w_arms )*
          Self::#fallback_ident(#fallback_substruct_name { w, .. }) => *w,
        }
      }

      pub const fn l(&self) -> LParam {
        match self {
          #( #l_arms )*
          Self::#fallback_ident(#fallback_substruct_name { l, .. }) => *l,
        }
      }
    }
  };

  output.into()
}
