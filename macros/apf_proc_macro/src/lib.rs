extern crate proc_macro;

// use crate::proc_macro;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::quote;
// use syn;

use std::iter::FromIterator;

#[proc_macro_attribute]
pub fn authorized_only(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // dbg!(attr);
    // println!("BEFORE:");
    // dbg!(&item);

    // let attr = proc_macro2::TokenStream::from(attr);

    // let mut it = attr.into_iter();

    // let role = match it.next().unwrap(){
    //     TokenTree::Ident(ident) => ident.to_string(),
    //     _ => panic!("no ident")
    // };

    // dbg!(role);

    // let item2:TokenTree = item.into();
    let mut func_name = String::new();

    // convert ke proc_macro2 dulu
    let item2 = proc_macro2::TokenStream::from(item);

    let items = item2.into_iter();

    #[allow(unused_assignments)]
    let mut no_add = false;

    let mut in_fn = 0;
    let mut after_fn = false;
    let mut group_cnt = 0;
    // let mut added_op = false;
    let mut has_http_req = false;
    let mut tb = vec![];

    for item in items {
        no_add = false;

        if item.to_string() == "fn" {
            in_fn = 1;
            tb.push(item);
            continue;
        }

        if in_fn == 1 && !after_fn {
            // in_fn = 2;
            after_fn = true;
            func_name = item.to_string();
            tb.push(item);
            continue;
        }

        if after_fn {
            match item {
                TokenTree::Group(ref group) => {
                    // let param = group.stream().into_iter().flat_map(|a| vec![a]);

                    for inner in group.stream() {
                        match inner {
                            TokenTree::Ident(ref ident) => {
                                if ident.to_string() == "HttpRequest" {
                                    has_http_req = true;
                                }
                            }
                            _ => (),
                        }
                    }

                    group_cnt += 1;
                }
                _ => (),
            }
            if group_cnt == 3 && in_fn < 2 {
                in_fn = 2;

                if !has_http_req {
                    panic!(
                        "Gagal menjadikan auth endpoint untuk fungsi `{}`, \
                         gunakan `endpoint_req_mut` untuk membuat endpoint yang terotorisasi.",
                        func_name
                    );
                }

                if let TokenTree::Group(ref group) = item {
                    let mut new_stream = vec![];
                    let access_token_guard: TokenStream = quote! {
                        // {
                            let access_token = req.headers().get("X-Access-Token")
                                .ok_or(api::Error::Unauthorized)?
                                .to_str()
                                .map_err(|_| api::Error::Unauthorized)?;

                            // println!("got access token: {}", access_token);
                        // }
                    };
                    new_stream.push(access_token_guard);
                    new_stream.push(group.stream());

                    let group = Group::new(
                        Delimiter::Brace,
                        TokenStream::from_iter(new_stream.into_iter()),
                    );
                    let tt: TokenTree = TokenTree::Group(group);
                    tb.push(tt);
                }
                continue;
            }
        }

        if !no_add {
            tb.push(item);
        }
    }

    // let func_name = Ident::new(&func_name, Span::call_site());

    // dbg!(&func_name);

    // let gen = quote! {
    //     // fn #func_name (state: &AppState, query: TxQuery<Credit>) -> api::Result<()> {
    //     //     trace!(concat!("API call /", #func_name, ", query: {:?}"), query);
    //     //     // @TODO(*): Code here
    //     //     Ok(())
    //     // }
    // };

    // item
    // gen.into()

    // after
    // println!("AFTER:");
    // dbg!(&tb2);

    proc_macro::TokenStream::from(TokenStream::from_iter(tb.into_iter()))
}
