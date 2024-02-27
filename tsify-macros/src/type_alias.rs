use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::{
    attrs::TsifyContainerAttrs, comments::extract_doc_comments, ctxt::Ctxt, decl::TsTypeAliasDecl,
    typescript::TsType,
};

pub fn expand(args: proc_macro2::TokenStream, item: syn::ItemType) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();

    let attribute = parse_quote!(#[tsify(#args)]);

    let attrs = TsifyContainerAttrs::from_attrs(&[attribute], None)?;

    let type_ann = TsType::from_syn_type(&attrs.ty_config, item.ty.as_ref());

    let decl = TsTypeAliasDecl {
        id: attrs.ty_config.format_name(item.ident.to_string()),
        export: true,
        type_params: item
            .generics
            .type_params()
            // While strictly speaking we shouldn't need to suffix the type parameters, the
            // type parsing code doesn't know if the type its looking at is referencing this
            // type's generic or some other type. So we just suffix everything
            .map(|ty| attrs.ty_config.format_name(ty.ident.to_string()))
            .collect(),
        type_ann,
        comments: extract_doc_comments(&item.attrs),
    };

    let decl_str = decl.to_string();

    let typescript_custom_section = quote! {
        #[automatically_derived]
        const _: () = {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen(typescript_custom_section)]
            const TS_APPEND_CONTENT: &'static str = #decl_str;
        };
    };

    ctxt.check()?;

    let tokens = quote! {
      #item
      #typescript_custom_section
    };

    Ok(tokens)
}
