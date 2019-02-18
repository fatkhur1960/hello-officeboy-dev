#![recursion_limit = "128"]
#![allow(unused_imports, unused_assignments, unused_mut)]

extern crate proc_macro;

// #[macro_use]
// extern crate darling;
#[macro_use]
extern crate syn;

// use crate::proc_macro;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;
// use syn;

use std::iter::FromIterator;

use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::io::prelude::*;
use std::{
    fs::{self, File, OpenOptions},
    io::LineWriter,
};

#[derive(Serialize, Debug)]
struct ApiEndpoint {
    pub path: String,
    pub rel_path: String,
    pub title: String,
    pub desc: String,
    pub method: String,
    pub method_name: String,
}

fn create_file(scope: &'static str) -> Arc<Mutex<File>> {
    let file_name = format!("api-docs/{}-endpoints.raw.txt", scope);
    println!("creating {} file", file_name);
    if fs::metadata(&file_name).is_ok() {
        fs::remove_file(&file_name).unwrap_or_else(|_| panic!("Cannot remove file {}", file_name));
    }
    Arc::new(Mutex::new(
        OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(&file_name)
            .expect("Cannot write api-docs.raw.txt"),
    ))
}

lazy_static! {
    // static ref API_DOC_TREE: Arc<Mutex<Vec<ApiEndpoint>>> = Arc::new(Mutex::new(vec![]));
    static ref FILE_PUBLIC:Arc<Mutex<File>> = {
        create_file("public")
    };
    static ref FILE_PRIVATE:Arc<Mutex<File>> = {
        create_file("private")
    };
    static ref CURRENT_SCOPE:Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

fn get_lit_str(lit: &proc_macro2::Literal) -> String {
    let a = lit.to_string();
    a[1..a.len() - 1].trim().to_string()
}

// use darling::FromMeta;
// use syn::{AttributeArgs, ItemFn};

// #[derive(Debug, FromMeta)]
// struct ApiMacroArgs {
//     pub path:String,
//     pub auth:String,
//     pub mutable:bool
// }

fn gather_endpoint_info(stream: TokenStream, base: &str) -> ApiEndpoint {
    let mut path = String::new();
    let mut mutable = false;

    let mut to_update = &mut path;
    let mut nicd = 0;

    // dbg!(&stream);

    for item in stream {
        match &item {
            TokenTree::Ident(ident) if ident.to_string() == "mutable" => {
                mutable = true;
            }
            TokenTree::Ident(ident) if ident.to_string() == "path" => {
                to_update = &mut path;
                nicd = 2;
            }
            TokenTree::Literal(lit) if nicd == 0 => {
                *to_update = get_lit_str(lit);
            }
            _ => (),
        }
        nicd = nicd - 1;
    }

    ApiEndpoint {
        path: format!("{}{}", base, path),
        rel_path: path,
        title: Default::default(),
        desc: Default::default(),
        method: if mutable {
            "POST".to_string()
        } else {
            "GET".to_string()
        },
        method_name: Default::default(),
    }
}

fn write_doc(api_scope: &str, text: &str) {
    let mut file = match api_scope {
        "public" => (*FILE_PUBLIC).lock().unwrap(),
        "private" => (*FILE_PRIVATE).lock().unwrap(),
        x => panic!("unknown scope: {}", x),
    };
    let _ = file.write(text.as_bytes());
}

// #[proc_macro_derive(ApiWire)]
// pub fn api_wire(input:proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let tb =
// }

#[proc_macro_attribute]
pub fn api_group(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);

    let mut group_name = String::new();
    let mut api_scope = String::new();
    let mut base = String::new();
    let mut struct_name = String::new();
    let mut _void = String::new();

    // dbg!(&attr);

    {
        let mut to_update = &mut _void;
        for item in attr {
            // dbg!(&item);
            match item {
                TokenTree::Ident(ident) => match ident.to_string().as_str() {
                    "base" => to_update = &mut base,
                    _ => (),
                },
                TokenTree::Literal(lit) => {
                    if group_name.is_empty() {
                        group_name = get_lit_str(&lit);
                    } else if api_scope.is_empty() {
                        api_scope = get_lit_str(&lit);
                    } else {
                        *to_update = get_lit_str(&lit);
                        // println!("{}", *to_update);
                    }
                }
                _ => (),
            }
        }
    }

    // dbg!(&attr);

    // {
    //     let mut file = match api_scope.as_str() {
    //         "public" => (*FILE_PUBLIC).lock().unwrap(),
    //         "private" => (*FILE_PRIVATE).lock().unwrap(),
    //         x => panic!("unknown scope: {}", x)
    //     };
    //     let _ = file.write(format!("## Group {}\n", group_name).as_bytes());
    // }
    write_doc(&api_scope, &format!("## Group {}\n", group_name));
    // let _ = file.write(format!("## Group {}\n", group_name).to_bytes());

    // *(*CURRENT_SCOPE).lock().unwrap() = api_scope;

    let mut api_endpoint_info = vec![];

    {
        let mut to_update = &mut _void;
        let items = proc_macro2::TokenStream::from(item.clone());

        for item in items {
            // dbg!(&item);

            match &item {
                TokenTree::Ident(ident) if ident.to_string() == "impl" => {
                    to_update = &mut struct_name;
                }
                TokenTree::Ident(ident) => {
                    *to_update = ident.to_string();
                    to_update = &mut _void;
                }
                TokenTree::Group(group) => {
                    let mut tb: Vec<TokenTree> = vec![];
                    let items = group.stream().into_iter();
                    let mut docs = vec![];

                    for item in items {
                        // dbg!(&item);
                        match &item {
                            TokenTree::Group(group) => {
                                let items = group.stream().into_iter();

                                let mut begin_doc = false;
                                let mut begin_api_endpoint = false;

                                for item in items {
                                    // dbg!(&item);
                                    match &item {
                                        // TokenTree::Ident(ident) => {
                                        //     dbg!(&ident);
                                        //     match ident.to_string().as_ref() {
                                        //         "doc" => {
                                        //             begin_doc = true;
                                        //         }
                                        //         _ => (),
                                        //     }
                                        // }
                                        TokenTree::Ident(ident) => {
                                            // dbg!(&ident);
                                            match ident.to_string().as_ref() {
                                                "doc" => {
                                                    begin_doc = true;
                                                }
                                                "api_endpoint" => {
                                                    begin_api_endpoint = true;
                                                }
                                                _ => (),
                                            }
                                        }
                                        TokenTree::Literal(lit) if begin_doc => {
                                            docs.push(get_lit_str(&lit));
                                        }
                                        TokenTree::Group(group) if begin_api_endpoint == true => {
                                            let mut info = gather_endpoint_info(group.stream(), &base);

                                            info.desc = docs.join("\n");
                                            docs = vec![];
                                            api_endpoint_info.push(info);

                                            begin_api_endpoint = false;
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            TokenTree::Ident(ident) => {
                                if tb[tb.len() - 1].to_string() == "fn" {
                                    println!("after fn");
                                    api_endpoint_info.last_mut().map(|info| {
                                        info.method_name = ident.to_string();
                                        dbg!(&info);
                                    });
                                }
                            }
                            _ => (),
                        }

                        tb.push(item.clone());
                    }
                }
                _ => (),
            }
        }

        // let api_endp_json = json!({
        //     "path": path,
        //     "title": "",
        //     "desc": docs.join(", "),
        //     "method": if is_mutable { "POST" }else{ "GET" }
        // });

        // let current_scope = (*(*CURRENT_SCOPE).lock().unwrap()).clone();

        // let mut file = match api_scope.as_str() {
        //     "public" => (*FILE_PUBLIC).lock().unwrap(),
        //     "private" => (*FILE_PRIVATE).lock().unwrap(),
        //     x => panic!("unknown scope: {}", x)
        // };

        for aei in &api_endpoint_info {
            // let _ = file.write(serde_json::to_string_pretty(&aei).unwrap().as_bytes());
            // let _ = file.write(b"\n");
            let text = format!("{}\n", serde_json::to_string_pretty(&aei).unwrap());
            write_doc(&api_scope, &text);
        }
    }

    let tts = {
        let struct_name = Ident::new(&struct_name, Span::call_site());
        let mut sas = vec![];
        for aei in &api_endpoint_info {
            let rel_path = {
                let s = aei.path.split("/").skip(2).collect::<Vec<&str>>();
                s.join("/")
            };
            let path = Literal::string(&rel_path);
            let method_name = Ident::new(&aei.method_name, Span::call_site());
            sas.push(if aei.method == "POST" {
                quote! {
                    sas.endpoint_mut(#path, #struct_name::#method_name);
                }
            } else {
                quote! {
                    sas.endpoint(#path, #struct_name::#method_name);
                }
            });
        }
        let sases = TokenStream::from_iter(sas.into_iter());
        quote! {
            impl #struct_name {
                #[doc(hidden)]
                pub fn new() -> Box<#struct_name> {
                    Box::new(#struct_name{})
                }
            }
            impl crate::api::ApiEndpointDef for #struct_name {
                fn wire(&self, sas: &mut crate::api::ServiceApiScope) {
                    #sases
                }
            }
        }
    };

    // item.extend(tts);

    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tts);

    // item
    proc_macro::TokenStream::from(item)
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

    // let file = (*FILE).lock().unwrap();

    // let mut lf = LineWriter::new(*file);

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
    let mut docs: Vec<String> = vec![];
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
                                    let docstr = lit.to_string();
                                    let real_doc = docstr[1..docstr.len() - 1].to_string().trim().to_string();
                                    docs.push(real_doc.clone());
                                    let text = format!(
                                        " API endpoint: `{}`<br/>Auth: {}<br/>Deskripsi: {}",
                                        path, auth_str, real_doc
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
