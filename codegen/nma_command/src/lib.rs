use nma_models::Command;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, Token, Lit};

struct CommandMacroWrapper {
    pub _inner_command: Command,
}

impl Parse for CommandMacroWrapper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let find = Lit::parse(input);

        let find_value = match find {
            Ok(v) => match v {
                Lit::Str(string) => string.value(),
                _ => return Err(syn::Error::new(v.span(), "Not a string!")),
            }
            Err(_) => "".to_string(),
        };

        let stop = <Token![!]>::parse(input);
        let _ = <Token![=>]>::parse(input)?;
        let change = Lit::parse(input);

        let change_value = match change {
            Ok(v) => match v {
                Lit::Str(string) => string.value(),
                _ => return Err(syn::Error::new(v.span(), "Not a string!")),
            }
            Err(_) => "".to_string(),
        };

        let stop_value = match stop {
            Ok(_) => true,
            Err(_) => false,
        };

        Ok(CommandMacroWrapper { _inner_command: Command::new(find_value, change_value, stop_value) })
    }
}

impl ToTokens for CommandMacroWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let fv = self._inner_command.find.clone().to_string();
        let cv = self._inner_command.change.clone().to_string();
        let sp = self._inner_command.exit;

        tokens.extend(quote::quote! {
            Command::new(String::from(#fv), String::from(#cv), #sp)
        });
    }
}



#[proc_macro]
pub fn command(input: TokenStream) -> TokenStream {
    let v = parse_macro_input!(input as CommandMacroWrapper);
    quote::quote! {
        #v
    }.into()
}