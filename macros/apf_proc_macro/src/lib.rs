#![recursion_limit = "128"]
#![allow(unused_imports, unused_assignments, unused_mut)]

extern crate proc_macro;

// use crate::proc_macro;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;
// use syn;

use std::iter::FromIterator;

#[proc_macro_attribute]
pub fn authorized_only(
    _attr: proc_macro::TokenStream,
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

    let mut func_name = String::new();

    // convert ke proc_macro2 dulu
    let item2 = proc_macro2::TokenStream::from(item);

    let items = item2.into_iter();

    #[allow(unused_assignments)]
    let mut no_add = false;

    let mut in_fn = 0;
    let mut after_fn = false;
    let mut group_cnt = 0;
    let mut in_open_fn = false;
    // let mut added_op = false;
    let mut has_http_req = false;
    let mut tb = vec![];

    for item in items {
        no_add = false;

        // dbg!(&item);

        if item.to_string() == "fn" {
            in_fn = 1;
            tb.push(item);
            continue;
        }

        if in_fn == 1 && !after_fn {
            after_fn = true;
            func_name = item.to_string();
            tb.push(item);
            continue;
        }

        // dbg!((group_cnt, after_fn, in_fn, has_http_req));

        if after_fn {
            match item {
                TokenTree::Group(ref group) => {
                    // let param = group.stream().into_iter().flat_map(|a| vec![a]);
                    for inner in group.stream() {
                        match inner {
                            TokenTree::Ident(ref ident) => {
                                if ident.to_string() == "ApiHttpRequest" {
                                    has_http_req = true;
                                }
                            }
                            _ => (),
                        }
                    }

                    group_cnt += 1;
                    in_open_fn = group.delimiter() == Delimiter::Brace;
                }
                _ => (),
            }

            if group_cnt > 1 && in_fn < 2 && in_open_fn {
                in_fn = 2;

                if !has_http_req {
                    panic!(
                        "Gagal menjadikan auth endpoint untuk fungsi `{}`, gunakan `endpoint_req_mut` untuk \
                         membuat endpoint yang terotorisasi.",
                        func_name
                    );
                }

                if let TokenTree::Group(ref group) = item {
                    let mut new_stream = vec![];
                    let access_token_guard: TokenStream = quote! {

                        let current_account = {
                            let access_token = req.headers().get("X-Access-Token")
                                .ok_or(ApiError::Unauthorized)?
                                .to_str()
                                .map_err(|_| ApiError::Unauthorized)?;

                            // periksa akses token
                            let schema = crate::auth::Schema::new(state.db());
                            let access_token = schema.get_access_token(&access_token)
                                .map_err(|_| ApiError::Unauthorized)?;

                            if access_token.expired(){
                                warn!("access token expired: {}", &access_token.token[..10]);
                                Err(ApiError::Expired("access token"))?
                            }

                            let account_schema = schema_op::Schema::new(state.db());
                            let account = account_schema.get_account(access_token.account_id)?;

                            account
                        };

                    };
                    new_stream.push(access_token_guard);
                    new_stream.push(group.stream());

                    let group = Group::new(Delimiter::Brace, TokenStream::from_iter(new_stream.into_iter()));
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

    // dbg!(&tb);

    proc_macro::TokenStream::from(TokenStream::from_iter(tb.into_iter()))
}

#[proc_macro_attribute]
pub fn api_endpoint(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // proses atribut
    let attr = proc_macro2::TokenStream::from(attr);

    let mut in_path = false;
    let mut in_auth = false;
    let mut path = "".to_string();
    let mut auth = 2;
    let mut auth_str = "required";
    let mut func_name = "".to_string();
    let mut is_mutable = false;
    let mut debug = false;

    // dbg!(&attr);

    for item in attr {
        match item {
            TokenTree::Ident(ident) => {
                in_path = ident.to_string() == "path";
                in_auth = ident.to_string() == "auth";
                is_mutable = is_mutable || ident.to_string() == "mutable";
            }
            TokenTree::Punct(_) => {}
            TokenTree::Literal(lit) => {
                if in_path {
                    in_path = false;
                    path = lit.to_string().replace("\"", "");
                }
                if in_auth {
                    in_auth = false;
                    match lit.to_string().as_ref() {
                        "\"optional\"" => {
                            auth = 1;
                            auth_str = "optional";
                        }
                        "\"required\"" => {
                            auth = 2;
                            auth_str = "required";
                        }
                        "\"none\"" => {
                            auth = 0;
                            auth_str = "none";
                        }
                        x => panic!(
                            "kebutuhan auth tidak dipahami: {}, hanya bisa salah satu dari: `optional`, \
                             `required`, dan `none`.",
                            x
                        ),
                    }
                }
            }
            _ => (),
        }
    }

    // dbg!((in_path, in_auth, auth_str, is_mutable));

    // println!("========= PATH: {} ============", path);
    // debug = path == "/account/register";
    // debug = path == "/transfer";
    // debug = false;

    // proses inner function
    // convert ke proc_macro2 dulu
    let item2 = proc_macro2::TokenStream::from(item);

    if debug {
        // dbg!(&item2);
    }

    let items = item2.into_iter();

    #[allow(unused_assignments)]
    let mut no_add = false;

    let mut in_fn = 0;
    let mut after_fn = false;
    let mut group_cnt = 0;
    let mut in_open_fn = false;
    let mut return_wrapped = false;
    let mut tb: Vec<TokenTree> = vec![];
    // let mut prev_token = TokenTree::Ident(Ident::new("a", Span::call_site()));
    let mut begin_capture_result_type = false;
    let mut result_type: Vec<TokenTree> = vec![];

    for item in items {
        no_add = false;

        if begin_capture_result_type {
            match &item {
                TokenTree::Group(ref group) => {
                    let end_capture = group.delimiter() == Delimiter::Brace;
                    begin_capture_result_type = !end_capture;
                    if end_capture {
                        let rettype = TokenStream::from_iter(result_type.clone().into_iter());
                        let new_return_type = quote! {
                            api::Result<#rettype>
                        };
                        for r in new_return_type {
                            tb.push(r);
                        }
                        return_wrapped = true;
                    }
                }
                _ => {
                    result_type.push(item.clone());
                    continue;
                }
            }
        }

        if item.to_string() == "fn" {
            in_fn = 1;
            // prev_token = item.clone();
            tb.push(item);
            continue;
        }

        if in_fn == 1 && !after_fn {
            after_fn = true;
            func_name = item.to_string();
            // prev_token = item.clone();
            tb.push(item);
            continue;
        }

        // dbg!((group_cnt, after_fn, in_fn, has_http_req));

        if !after_fn {
            let mut in_doc = false;
            match &item {
                TokenTree::Group(ref group) => {
                    for inner in group.stream() {
                        match inner {
                            TokenTree::Ident(ref ident) => {
                                if ident.to_string() == "doc" {
                                    in_doc = true;
                                }
                            }
                            TokenTree::Literal(lit) => {
                                if in_doc {
                                    let text = format!(
                                        " API endpoint: `{}`<br/>Auth: {}<br/>Deskripsi: {}",
                                        path,
                                        auth_str,
                                        lit.to_string().replace("\"", "")
                                    );
                                    let doc = TokenTree::Group(Group::new(
                                        Delimiter::Bracket,
                                        quote! { doc = #text },
                                    ));
                                    tb.push(doc);
                                }
                            }
                            _ => (),
                        }
                    }
                }

                _ => (),
            }
            if in_doc {
                continue;
            }
        }

        if after_fn {
            let mut query_type: Vec<TokenTree> = vec![];
            match item {
                TokenTree::Group(ref group) => {
                    group_cnt += 1;
                    in_open_fn = group.delimiter() == Delimiter::Brace;

                    if group_cnt == 1 {
                        if let TokenTree::Group(ref group) = item {
                            let mut in_query = false;
                            let mut begin_capture_query_type = false;
                            for inner in group.stream() {
                                match inner {
                                    TokenTree::Ident(ref ident) => {
                                        if ident.to_string() == "query" {
                                            in_query = true;
                                        } else if in_query {
                                            in_query = false;
                                            begin_capture_query_type = true;
                                            query_type.push(inner.clone());
                                        } else if begin_capture_query_type {
                                            query_type.push(inner.clone());
                                        }
                                    }
                                    TokenTree::Group(ref g) => {
                                        if in_query
                                            && g.delimiter() == Delimiter::Parenthesis
                                            && !begin_capture_query_type
                                        {
                                            in_query = false;
                                            query_type.push(inner.clone());
                                        } else if begin_capture_query_type {
                                            query_type.push(inner.clone());
                                        }
                                    }
                                    TokenTree::Punct(ref punct) => {
                                        if begin_capture_query_type {
                                            if punct.to_string() == "," {
                                                begin_capture_query_type = false;
                                            } else {
                                                query_type.push(inner.clone());
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }

                        if query_type.is_empty() {
                            panic!(
                                "API endpoint `{}` pada fungsi `{}` perlu ada `query` parameter-nya, \
                                 contoh: `pub {}(query: Query) -> JsonValue`.",
                                path, func_name, func_name
                            );
                        }

                        let query_type = TokenStream::from_iter(query_type.into_iter());

                        if debug {
                            // dbg!(&query_type);
                        }

                        let group = Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::from_iter(
                                (if is_mutable {
                                    quote! {
                                        state: &mut AppState, query: #query_type, req: &api::HttpRequest
                                    }
                                } else {
                                    quote! {
                                        state: &AppState, query: #query_type, req: &api::HttpRequest
                                    }
                                })
                                .into_iter(),
                            ),
                        );
                        let tt: TokenTree = TokenTree::Group(group);
                        tb.push(tt);
                        // prev_token = item.clone();
                        continue;
                    }
                }
                _ => (),
            }
            if group_cnt >= 1 && !return_wrapped {
                // wrap return value menggunakan ApiResult<>
                match (&tb.get(tb.len() - 2), &tb.last()) {
                    (Some(&TokenTree::Punct(ref punct1)), Some(&TokenTree::Punct(ref punct2))) => {
                        if punct1.as_char() == '-' && punct2.as_char() == '>' {
                            begin_capture_result_type = true;
                            result_type.push(item.clone());
                            continue;
                        }
                    }
                    _ => (),
                }
            }

            if group_cnt > 1 && in_fn < 2 && in_open_fn {
                in_fn = 2;

                if let TokenTree::Group(ref group) = item {
                    let mut new_stream = vec![];

                    if auth != 0 {
                        // selain `none`
                        let access_token_guard: TokenStream = quote! {
                            use crate::valid::Expirable;
                            let current_account = req.headers().get("X-Access-Token")
                                .map(|at| {
                                    let schema = crate::auth::Schema::new(state.db());
                                    schema.get_access_token(at.to_str().unwrap())
                                        .map(|at|{
                                            if !at.expired(){
                                                let account_schema = crate::schema_op::Schema::new(state.db());
                                                account_schema.get_account(at.account_id)
                                                    .map_err(api::Error::from)
                                            }else{
                                                Err(api::Error::Expired("access token"))
                                            }
                                        })
                                        .map_err(|_| api::Error::Unauthorized)
                                });
                        };

                        new_stream.push(access_token_guard);
                    }

                    match auth {
                        2 => {
                            // required
                            let access_token_unwraper = quote! {
                                let current_account = match current_account {
                                    Some(r) => r??,
                                    None => Err(api::Error::Unauthorized)?
                                };
                            };
                            new_stream.push(access_token_unwraper);
                        }
                        1 => {
                            // optional
                            let access_token_unwraper = quote! {
                                let current_account = match current_account {
                                    Some(Ok(Ok(a))) => Some(a),
                                    _ => None
                                };
                            };
                            new_stream.push(access_token_unwraper);
                        }
                        _ => (), // none
                    }

                    new_stream.push(group.stream());

                    let group = Group::new(Delimiter::Brace, TokenStream::from_iter(new_stream.into_iter()));
                    let tt: TokenTree = TokenTree::Group(group);
                    tb.push(tt);
                }
                // prev_token = item.clone();
                continue;
            }
        }

        if !no_add {
            // prev_token = item.clone();
            tb.push(item);
        }
    }

    if debug {
        // dbg!(&tb);
    }

    proc_macro::TokenStream::from(TokenStream::from_iter(tb.into_iter()))
}
