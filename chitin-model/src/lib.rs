use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemMod};

#[proc_macro_attribute]
pub fn chitin_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod: &ItemMod = &parse_macro_input!(item as ItemMod);
    let mut structs = Vec::new();
    match item_mod.content {
        Some((_, ref contents)) => {
            for content in contents {
                match content {
                    Item::Struct(item_struct) => {
                        structs.push(item_struct.ident.clone());
                    }
                    _ => {}
                }
            }
        }
        None => {
            panic!("mode 沒有 content");
        }
    }
    let ident = &item_mod.ident;
    let attrs = &item_mod.attrs;
    let (_, ref content) = item_mod.content.as_ref().unwrap();
    let new_mod = quote! {
        #(#attrs)* mod #ident {
            #(#content)*
            pub fn gen_typescript() -> String {
                let mut ret = String::new();
                #(
                    ret.push_str(&#structs::type_script_ify());
                    ret.push('\n');
                )*
                ret
            }
        }
    };
    // println!("new_mod = {} $", new_mod.to_string());
    TokenStream::from(new_mod)
}
