#![recursion_limit = "128"]

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
                            let schema = auth::Schema::new(state.db());
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
pub fn api_endpoint(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // proses atribut
    let attr = proc_macro2::TokenStream::from(attr);

    let mut in_path = false;
    let mut in_auth = false;
    let mut path = "".to_string();
    let mut auth = 1;
    let mut auth_str = "";
    let mut func_name = "".to_string();

    for item in attr {
        match item {
            TokenTree::Ident(ident) => {
                in_path = ident.to_string() == "path";
                in_auth = ident.to_string() == "auth";
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
                            auth = 0;
                            auth_str = "optional";
                        }
                        "\"required\"" => {
                            auth = 1;
                            auth_str = "required";
                        }
                        _ => {
                            auth = 1;
                            auth_str = "required";
                        }
                    }
                }
            }
            _ => (),
        }
    }

    // proses inner function
    // convert ke proc_macro2 dulu
    let item2 = proc_macro2::TokenStream::from(item);

    // dbg!(&item2);

    let items = item2.into_iter();

    #[allow(unused_assignments)]
    let mut no_add = false;

    let mut in_fn = 0;
    let mut after_fn = false;
    let mut group_cnt = 0;
    let mut in_open_fn = false;
    let mut tb = vec![];
    let mut prev_token = TokenTree::Ident(Ident::new("a", Span::call_site()));

    for item in items {
        no_add = false;

        if item.to_string() == "fn" {
            in_fn = 1;
            prev_token = item.clone();
            tb.push(item);
            continue;
        }

        if in_fn == 1 && !after_fn {
            after_fn = true;
            func_name = item.to_string();
            prev_token = item.clone();
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
            let mut query_type = "".to_string();
            match item {
                TokenTree::Group(ref group) => {
                    group_cnt += 1;
                    in_open_fn = group.delimiter() == Delimiter::Brace;

                    if group_cnt == 1 {
                        if let TokenTree::Group(ref group) = item {
                            let mut in_query = false;
                            for inner in group.stream() {
                                match inner {
                                    TokenTree::Ident(ref ident) => {
                                        if ident.to_string() == "query" {
                                            in_query = true;
                                        } else if in_query {
                                            in_query = false;
                                            query_type = ident.to_string();
                                        }
                                    }
                                    TokenTree::Group(g) => {
                                        if in_query && g.delimiter() == Delimiter::Parenthesis {
                                            in_query = false;
                                            query_type = "()".to_string()
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }

                        if query_type == "" {
                            panic!(
                                "API endpoint `{}` pada fungsi `{}` perlu ada `query` parameter-nya, \
                                 contoh: `pub {}(query: Query) -> JsonValue`.",
                                path, func_name, func_name
                            );
                        }

                        let query_type = {
                            if query_type == "()" {
                                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new()))
                            } else {
                                TokenTree::Ident(Ident::new(&query_type, Span::call_site()))
                            }
                        };

                        let group = Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::from_iter(
                                quote! {
                                    state: &AppState, query: #query_type, req: &ApiHttpRequest
                                }
                                .into_iter(),
                            ),
                        );
                        let tt: TokenTree = TokenTree::Group(group);
                        tb.push(tt);
                        prev_token = item.clone();
                        continue;
                    }
                }
                _ => (),
            }

            if group_cnt == 1 {
                // dbg!(&prev_token);
                // wrap return value menggunakan ApiResult<>
                match &prev_token {
                    &TokenTree::Punct(ref punct) => {
                        if punct.as_char() == '>' {
                            tb.push(TokenTree::Ident(Ident::new("ApiResult", Span::call_site())));
                            tb.push(TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                            tb.push(item.clone());
                            tb.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
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
                    let access_token_guard: TokenStream = quote! {

                        let current_account = req.headers().get("X-Access-Token")
                            .map(|at| {
                                let schema = auth::Schema::new(state.db());
                                schema.get_access_token(at.to_str().unwrap())
                                    .map(|at|{
                                        if !at.expired(){
                                            let account_schema = schema_op::Schema::new(state.db());
                                            account_schema.get_account(at.account_id)
                                                .map_err(ApiError::from)
                                        }else{
                                            Err(ApiError::Expired("access token"))
                                        }
                                    }).map_err(ApiError::from)
                            });
                    };

                    new_stream.push(access_token_guard);

                    if auth == 1 {
                        // required
                        let access_token_unwraper = quote! {
                            let current_account = match current_account {
                                Some(r) => r??,
                                None => Err(ApiError::Unauthorized)?
                            };
                        };
                        new_stream.push(access_token_unwraper);
                    } else if auth == 0 {
                        // optional
                        let access_token_unwraper = quote! {
                            let current_account = match current_account {
                                Some(Ok(Ok(a))) => Some(a),
                                _ => None
                            };
                        };
                        new_stream.push(access_token_unwraper);
                    }

                    new_stream.push(group.stream());

                    let group = Group::new(Delimiter::Brace, TokenStream::from_iter(new_stream.into_iter()));
                    let tt: TokenTree = TokenTree::Group(group);
                    tb.push(tt);
                }
                prev_token = item.clone();
                continue;
            }
        }

        if !no_add {
            prev_token = item.clone();
            tb.push(item);
        }
    }

    // dbg!(&tb);

    proc_macro::TokenStream::from(TokenStream::from_iter(tb.into_iter()))
}
