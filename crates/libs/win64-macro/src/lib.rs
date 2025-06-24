mod message;

#[proc_macro_derive(Message, attributes(id, params, fallback))]
pub fn message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  self::message::macro_impl(input)
}
