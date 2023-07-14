use proc_macro::TokenStream;
use syn::{parse_macro_input, parse::Parse, Token, LitStr, Expr, punctuated::Punctuated};
use quote::quote;

struct FormKeyValue {
    key: LitStr,
    value: Expr
}

impl Parse for FormKeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![:]>()?;
        let value = input.parse()?;

        Ok(Self {
            key, value
        })
    }
}

struct FormData {
    key_values: Punctuated<FormKeyValue, Token![,]>,
}

impl Parse for FormData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key_values = Punctuated::parse_separated_nonempty(input)?;

        Ok(Self {
            key_values
        })
    }
}

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FormData);

    let (mut keys, mut values): (Vec<_>, Vec<_>) = input.key_values.into_iter().map(|kv| (kv.key, kv.value)).unzip();
    let last_key = keys.pop().unwrap();
    let last_value = values.pop().unwrap();

    let tokens = quote! {
        {
            let mut builder = string_builder::Builder::default();
            #(
                builder.append(#keys);
                builder.append("=");
                builder.append(#values.to_string());
                
                builder.append("&");
            )*
            
            builder.append(#last_key);
            builder.append("=");
            builder.append(#last_value.to_string());
            
            let mut result = builder.string().unwrap();
            result
        }
    };

    tokens.into()
}
