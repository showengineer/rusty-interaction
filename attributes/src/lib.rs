extern crate proc_macro;

use proc_macro::*;

use quote::format_ident;
use quote::quote;

use syn::{Expr, ExprReturn, FnArg, ReturnType, Stmt};

fn handler(
    _attr: TokenStream,
    item: TokenStream,
    defer_return: quote::__private::TokenStream,
) -> TokenStream {
    // There is _probably_ a more efficient way to do what I want to do, but hey I am here
    // to learn so why not join me on my quest to create this procedural macro...lol
    let mut defer = false;

    // Parse the stream of tokens to something more usable.
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    // Let's see if the programmer wants to respond with a deferring acknowlegdement first.
    // If so, the end-result needs to be built differently.
    for at in &input.attrs {
        for seg in at.path.segments.clone() {
            if seg.ident == "defer" {
                defer = true;
            }
        }
    }

    // Ok here comes the fun part

    // Get the function name
    let fname = &input.sig.ident;
    // Get the visibility (public fn, private fn, etc)
    let vis = &input.vis;

    // Get the parameters and return types
    let params = &input.sig.inputs;
    let ret_sig = &input.sig.output;

    // Must be filled later, but define its type for now.
    let ret: syn::Type;

    // Get the function body
    let body = &input.block;

    // Check for a proper return type and fill ret if found.
    match ret_sig {
        ReturnType::Default => {
            panic!("Expected an `Result<InteractionResponse, ()>` return type, but got no return type. Consider adding `-> Result<InteractionResponse, ()>` to your function signature.");
        }
        ReturnType::Type(_a, b) => {
            ret = *b.clone();
        }
    }

    // Find the name of the Context parameter
    let mut ctxname: Option<syn::Ident> = None;
    let mut handlename: Option<syn::Ident> = None;
    // eprintln!("{:#?}", params);

    // I am honestly laughing at this...
    // But hey it works! :D
    for p in params {
        if let FnArg::Typed(t) = p {
            match &*t.ty {
                // This might be a Context
                syn::Type::Path(b) => {
                    for segment in b.path.segments.clone() {
                        if segment.ident == "Context" {
                            if let syn::Pat::Ident(a) = &*t.pat {
                                ctxname = Some(a.ident.clone());
                                break;
                            }
                        } else if segment.ident == "InteractionHandler" {
                            panic!("Cannot take ownership of `InteractionHandler`. Try using &InteractionHandler!")
                        }
                    }
                }
                // This might be an &InteractionHandler!
                syn::Type::Reference(r) => {
                    let e = r.elem.clone();
                    if let syn::Type::Path(w) = &*e {
                        for segment in w.path.segments.clone() {
                            if segment.ident == "InteractionHandler" {
                                if let syn::Pat::Ident(a) = &*t.pat {
                                    handlename = Some(a.ident.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        if let FnArg::Receiver(_) = p {
            panic!("`self` arguments are not allowed. If you need to access data in your function, use the `InteractionHandler.data` field and `InteractionHandler::add_data` method")
        }
    }

    if ctxname.is_none() {
        panic!("Couldn't determine the Context parameter. Make sure you take a `Context` as an argument");
    }

    let mut ih_n = quote!(_);

    if handlename.is_some() {
        ih_n = quote!(#handlename);
    }

    // Using quasi-quoting to generate a new function. This is what will be the end function returned to the compiler.
    if !defer {
        // Build the function
        let subst_fn = quote! {
            #vis fn #fname (#ih_n: &mut InteractionHandler, #ctxname: Context) -> ::std::pin::Pin<::std::boxed::Box<dyn Send + ::std::future::Future<Output = #ret> + '_>>{
                Box::pin(async move {
                    #body
                })
            }
        };
        subst_fn.into()
    }
    // Deferring is requested, this will require a bit more manipulation.
    else {
        // Create two functions. One that actually does the work, and one that handles the threading.

        let act_fn = format_ident!("__actual_{}", fname);

        let subst_fn = quote! {
            fn #act_fn (#ih_n: &mut InteractionHandler, #ctxname: Context) -> ::std::pin::Pin<::std::boxed::Box<dyn Send + ::std::future::Future<Output = #ret> + '_>>{
                Box::pin(async move {
                    #body
                })
            }
            #vis fn #fname (ihd: &mut InteractionHandler, ctx: Context) -> ::std::pin::Pin<::std::boxed::Box<dyn Send + ::std::future::Future<Output = #ret> + '_>>{
                Box::pin(async move {
                    // TODO: Try to do this without cloning.
                    let mut __ih_c = ihd.clone();

                    ::rusty_interaction::actix::Arbiter::spawn(async move {

                        let __response = #act_fn (&mut __ih_c, ctx.clone()).await;
                        if let Ok(__r) = __response{
                            if __r.r#type != InteractionResponseType::Pong && __r.r#type != InteractionResponseType::None{
                                if let Err(i) = ctx.edit_original(&WebhookMessage::from(__r)).await{
                                    ::rusty_interaction::log::error!("Editing original message failed: {:?}", i);
                                }
                            }
                        }
                        else{
                            // Nothing
                        }


                    });

                    return InteractionResponseBuilder::default().respond_type(#defer_return).finish();

                })
            }
        };
        subst_fn.into()
    }
}

#[proc_macro_attribute]
/// Convenience procedural macro that allows you to bind an async function to the [`InteractionHandler`] for handling component interactions.
pub fn component_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ret = quote!(
        ::rusty_interaction::types::interaction::InteractionResponseType::DefferedUpdateMessage
    );

    handler(attr, item, ret)
}

#[proc_macro_attribute]
/// Convenience procedural macro that allows you to bind an async function to the [`InteractionHandler`]
pub fn slash_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ret = quote!(::rusty_interaction::types::interaction::InteractionResponseType::DefferedChannelMessageWithSource);

    handler(attr, item, ret)
}

#[proc_macro_attribute]
/// Send out a deffered channel message response before doing work.
pub fn defer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[doc(hidden)]
#[proc_macro_attribute]
#[doc(hidden)]
// This is just here to make the tests work...lol
pub fn slash_command_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // There is _probably_ a more efficient way to do what I want to do, but hey I am here
    // to learn so why not join me on my quest to create this procedural macro...lol
    let mut defer = false;

    // Parse the stream of tokens to something more usable.
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    // Let's see if the programmer wants to respond with a deferring acknowlegdement first.
    // If so, the end-result needs to be built differently.
    for at in &input.attrs {
        for seg in at.path.segments.clone() {
            if seg.ident == "defer" {
                defer = true;
            }
        }
    }

    // Ok here comes the fun part

    // Get the function name
    let fname = &input.sig.ident;
    // Get the visibility (public fn, private fn, etc)
    let vis = &input.vis;

    // Get the parameters and return types
    let params = &input.sig.inputs;
    let ret_sig = &input.sig.output;

    // Must be filled later, but define its type for now.
    let ret: syn::Type;

    // Get the function body
    let body = &input.block;

    // Check for a proper return type and fill ret if found.
    match ret_sig {
        ReturnType::Default => {
            panic!("Expected an `InteractionResponse` return type, but got no return type. Consider adding `-> InteractionResponse` to your function signature.");
        }
        ReturnType::Type(_a, b) => {
            ret = *b.clone();
        }
    }

    // Find the name of the Context parameter
    let mut ctxname: Option<syn::Ident> = None;
    let mut handlename: Option<syn::Ident> = None;
    // eprintln!("{:#?}", params);

    // I am honestly laughing at this...
    // But hey it works! :D
    for p in params {
        if let FnArg::Typed(t) = p {
            match &*t.ty {
                // This might be a Context
                syn::Type::Path(b) => {
                    for segment in b.path.segments.clone() {
                        if segment.ident == "Context" {
                            if let syn::Pat::Ident(a) = &*t.pat {
                                ctxname = Some(a.ident.clone());
                                break;
                            }
                        }
                    }
                }
                // This might be an &InteractionHandler!
                syn::Type::Reference(r) => {
                    let e = r.elem.clone();
                    if let syn::Type::Path(w) = &*e {
                        for segment in w.path.segments.clone() {
                            if segment.ident == "InteractionHandler" {
                                if let syn::Pat::Ident(a) = &*t.pat {
                                    handlename = Some(a.ident.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }

    if ctxname.is_none() {
        panic!("Couldn't determine the Context parameter. Make sure you take a `Context` as an argument");
    }

    let mut ih_n = quote!(_);

    if handlename.is_some() {
        ih_n = quote!(#handlename);
    }

    // Using quasi-quoting to generate a new function. This is what will be the end function returned to the compiler.
    if !defer {
        // Build the function
        let subst_fn = quote! {
            #vis fn #fname (#ih_n: &mut InteractionHandler, #ctxname: Context) -> ::std::pin::Pin<::std::boxed::Box<dyn Send + ::std::future::Future<Output = #ret> + '_>>{
                Box::pin(async move {
                    #body
                })
            }
        };
        subst_fn.into()
    }
    // Deferring is requested, this will require a bit more manipulation.
    else {
        // Find the return statement and split the entire tokenstream there.
        let mut ind: Option<usize> = None;
        let mut expr: Option<ExprReturn> = None;
        for n in 0..body.stmts.len() {
            let s = &body.stmts[n];
            match s {
                Stmt::Expr(Expr::Return(a)) => {
                    expr = Some(a.clone());
                    ind = Some(n);
                    break;
                }
                Stmt::Semi(Expr::Return(a), _) => {
                    expr = Some(a.clone());
                    ind = Some(n);
                    break;
                }
                _ => (),
            }
        }
        let (nbody, _reta) = body.stmts.split_at(ind.unwrap_or_else(|| {
            panic!(
                "Could not find return statement in slash-command. Explicit returns are required."
            );
        }));

        // Unwrap, unwrap, unwrap, unwrap.
        let expra = expr
            .unwrap_or_else(|| panic!("Expected return"))
            .expr
            .unwrap_or_else(|| panic!("Expected some return value"));

        let nvec = nbody.to_vec();

        // Now that we have all the information we need, we can finally start building our new function!
        // The difference here being that the non-deffered function doesn't have to spawn a new thread that
        // does the actual work. Here we need it to reply with a deffered channel message.
        let subst_fn = quote! {
            #vis fn #fname (#ih_n: &mut InteractionHandler, #ctxname: Context) -> ::std::pin::Pin<::std::boxed::Box<dyn Send + ::std::future::Future<Output = #ret> + '_>>{
                Box::pin(async move {
                    actix::Arbiter::spawn(async move {
                        #(#nvec)*
                        if #expra.r#type != InteractionResponseType::Pong && #expra.r#type != InteractionResponseType::None{
                            if let Err(i) = #ctxname.edit_original(&WebhookMessage::from(#expra)).await{
                                error!("Editing original message failed: {:?}", i);
                            }
                        }

                    });

                    return InteractionResponseBuilder::default().respond_type(InteractionResponseType::DefferedChannelMessageWithSource).finish();

                })
            }
        };
        subst_fn.into()
    }
}
