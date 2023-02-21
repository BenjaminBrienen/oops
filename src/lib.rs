use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro]
pub fn make_line(stream: TokenStream) -> TokenStream
{
	let mut new = stream.clone();
	new.extend(stream);
	new
}
