use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Expr, ExprTuple, Ident, Variant, parse_macro_input};

fn wm_const(variant: &Variant) -> proc_macro2::TokenStream {
  // let wm = Ident::new(&id, Span::call_site());
  let wm = match variant.attrs.iter().find(|a| {
    let ident = a.path().get_ident().unwrap();
    a.path().is_ident(&Ident::new("id", ident.span()))
  }) {
    None => {
      let mut id = String::from("WM_");
      id.push_str(&variant.ident.to_string().to_case(convert_case::Case::UpperFlat));
      let wm = Ident::new(&id, Span::call_site());
      quote! { windows::Win32::UI::WindowsAndMessaging::#wm }
    }
    Some(a) => {
      //
      let attr: Expr = a.parse_args().unwrap();
      quote! { #attr }
    }
  };
  wm
}

#[proc_macro_derive(Id, attributes(id))]
pub fn message_id(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

  let variants = if let syn::Data::Enum(e) = data {
    e.variants
      .into_iter()
      .filter(|v| {
        v.ident != "Other" // Skip this one as it is a special case
      })
      .collect::<Vec<_>>()
  } else {
    panic!("Can only derive `Getter` on enums.");
  };

  let from_u32_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let wm = wm_const(v);
    // let wm: ItemConst = attr.parse_args().unwrap();
    quote! {
      #wm => Self::#variant_ident,
    }
  });

  let to_u32_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    let wm = wm_const(v);
    // let wm: ItemConst = attr.parse_args().unwrap();
    quote! {
      Self::#variant_ident => #wm,
    }
  });

  let expanded = quote! {
    impl From<u32> for #ident {
      fn from(msg: u32) -> Self {
        match msg {
          #( #from_u32_arms )*
          id => Self::Other(id),
        }
      }
    }
    
    impl #ident {
      pub const fn to_u32(self) -> u32 {
        match self {
          #( #to_u32_arms )*
          Self::Other(id) => id,
        }
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}

#[proc_macro_derive(GetId, attributes(id))]
pub fn get_id(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

  let variants = if let syn::Data::Enum(e) = data {
    e.variants
      .into_iter()
      .filter(|v| {
        v.ident != "Other" // Skip this one as it is a special case
      })
      .collect::<Vec<_>>()
  } else {
    panic!("Can only derive `Getter` on enums.");
  };

  let variant_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    // let wm = wm_const(v);
    // let wm: ItemConst = attr.parse_args().unwrap();
    quote! {
      Self::#variant_ident { .. } => MessageId::#variant_ident,
    }
  });

  let expanded = quote! {
    impl #ident {
      pub const fn id(&self) -> MessageId {
        match self {
          #( #variant_arms )*
          Self::Other { id, .. } => *id,
        }
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}

#[proc_macro_derive(Getter, attributes(getters))]
pub fn from_raw_message(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, data, attrs, .. } = parse_macro_input!(input as DeriveInput);

  let variants = if let syn::Data::Enum(e) = data {
    e.variants
      .into_iter()
      // .filter(|v| {
      //   v.ident != "Other" // Skip this one as it is a special case
      // })
      .collect::<Vec<_>>()
  } else {
    panic!("Can only derive `Getter` on enums.");
  };

  let mut stream = TokenStream::new();
  for attr in attrs {
    let mut attr: ExprTuple = attr.parse_args().unwrap();
    let field_type = attr.elems.pop().unwrap();
    let field_name = attr.elems.pop().unwrap();
    stream.extend(generate_function(&variants, &ident, field_name.value(), field_type.value()));
  }
  stream
}

fn generate_function(variants: &[Variant], ident: &Ident, field_name: &Expr, field_type: &Expr) -> TokenStream {
  // let field = Ident::new(field_name, Span::mixed_site());
  // let field_type = Ident::new(field_type, Span::mixed_site());

  let variant_arms = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    quote! {
      Self::#variant_ident { #field_name, .. } => *#field_name,
    }
  });

  let expanded = quote! {
    impl #ident {
      pub const fn #field_name(&self) -> #field_type {
        match self {
          #( #variant_arms )*
          // Self::Other { #field } => #field,
        }
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}
