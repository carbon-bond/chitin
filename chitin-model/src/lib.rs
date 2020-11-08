use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemMod, UseTree};

#[proc_macro_attribute]
pub fn chitin_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod: &ItemMod = &parse_macro_input!(item as ItemMod);
    let mut models = Vec::new();
    match item_mod.content {
        Some((_, ref contents)) => {
            for content in contents {
                match content {
                    Item::Struct(item_struct) => {
                        models.push(item_struct.ident.clone());
                    }
                    Item::Enum(item_enum) => {
                        models.push(item_enum.ident.clone());
                    }
                    Item::Use(item_used) => {
                        let attr = item_used.attrs.iter().find(|attr| {
                            attr.path
                                .get_ident()
                                .map_or(false, |ident| &ident.to_string() == "chitin_model_use")
                        });
                        if attr.is_none() {
                            continue;
                        }
                        let idents = extract_use_ident(&item_used.tree)
                            .expect("chitin_model_use 目前只支援單一名字");
                        models.extend(idents);
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
                    let ty = chitin_util::type_convert(&#models::type_script_ify());
                    ret.push_str(&ty);
                    ret.push('\n');
                )*
                ret
            }
        }
    };
    // println!("new_mod = {} $", new_mod.to_string());
    TokenStream::from(new_mod)
}

#[proc_macro_attribute]
pub fn chitin_model_use(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn extract_use_ident(mut tree: &UseTree) -> Result<Vec<Ident>, ()> {
    let mut idents = vec![];
    loop {
        match tree {
            UseTree::Path(path) => tree = path.tree.as_ref(),
            UseTree::Name(name) => {
                idents.push(name.ident.clone());
                break;
            }
            _ => return Err(()),
        }
    }
    Ok(idents)
}
