use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemMod, UseTree};

#[proc_macro_attribute]
pub fn chitin_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod: &ItemMod = &parse_macro_input!(item as ItemMod);
    let mut models = Vec::new();
    let mut inner_mods = Vec::new();
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
                    Item::Use(item_use) => {
                        let attr = item_use.attrs.iter().find(|attr| {
                            attr.path
                                .get_ident()
                                .map_or(false, |ident| &ident.to_string() == "chitin_model_use")
                        });
                        if attr.is_none() {
                            continue;
                        }
                        extract_use_ident(&mut models, &item_use.tree)
                            .expect("chitin_model_use 目前只支援單一名字");
                    }
                    Item::Mod(item_mod) => {
                        inner_mods.push(item_mod.ident.clone());
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
    let vis = &item_mod.vis;
    let (_, ref content) = item_mod.content.as_ref().unwrap();
    #[cfg(debug_assertions)]
    let new_mod = quote! {
        #(#attrs)* #vis mod #ident {
            #(#content)*
            pub fn gen_typescript() -> String {
                let mut ret = String::new();
                #(
                    let ty = chitin_util::type_convert(&#models::type_script_ify());
                    ret.push_str(&ty);
                    ret.push('\n');
                )*
                #(
                    let mod_name = std::stringify!(#inner_mods);
                    ret.push_str(&format!("export namespace {} ", mod_name));
                    ret.push_str("{\n");
                    let inner_mod = #inner_mods::gen_typescript();
                    ret.push_str(&inner_mod);
                    ret.push_str("}\n");
                )*
                ret
            }
        }
    };
    #[cfg(not(debug_assertions))]
    let new_mod = quote! {
        #(#attrs)* #vis mod #ident {
            #(#content)*
        }
    };
    // println!("new_mod = {} $", new_mod.to_string());
    TokenStream::from(new_mod)
}

#[proc_macro_attribute]
pub fn chitin_model_use(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn extract_use_ident(idents: &mut Vec<Ident>, tree: &UseTree) -> Result<(), ()> {
    match tree {
        UseTree::Path(path) => {
            extract_use_ident(idents, path.tree.as_ref())?;
        }
        UseTree::Rename(rename) => idents.push(rename.rename.clone()),
        UseTree::Name(name) => idents.push(name.ident.clone()),
        UseTree::Group(group) => {
            for item in group.items.iter() {
                extract_use_ident(idents, item)?;
            }
        }
        _ => return Err(()),
    }
    Ok(())
}
