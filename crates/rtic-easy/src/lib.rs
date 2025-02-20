extern crate proc_macro;

use std::hint::unreachable_unchecked;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    bracketed, parenthesized,
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Ident, Item, ItemFn, ItemMod, ItemStatic, LitInt, Token, Type,
};

enum Stmt {
    Ident(Ident),
    LockFree(Ident),
    Assign(Ident, Type, proc_macro2::TokenStream),
}

#[proc_macro]
pub fn if_should_analyse(item: TokenStream) -> TokenStream {
    if std::env::var("SYMEX").is_ok_and(|el| &el == "true") {
        return item;
    }
    TokenStream::new()
}

impl Parse for Stmt {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let borrow = input.peek(Token![&]);

        if borrow {
            let _: Token!(&) = input.parse()?;
        }

        let ident = input.parse()?;

        println!("Input :  {input}");
        if borrow {
            println!("Was lock free");
            return Ok(Self::LockFree(ident));
        }
        println!("Input :  {input}");
        if input.peek(Token!(:)) {
            let _ = input.parse::<Token![:]>()?;
            println!("Input :  {input}");
            let ty = input.parse::<Type>()?;

            println!(
                "Input :  {input}, found type {}",
                ty.to_token_stream().to_string()
            );
            let _ = input.parse::<Token![=]>()?;
            println!("Input :  {input}");
            let rhs = input.parse::<proc_macro2::TokenStream>()?;
            println!("Was assign");

            return Ok(Self::Assign(ident, ty, rhs));
        }
        println!("Was ident");
        Ok(Self::Ident(ident))
    }
}

#[derive(Default)]
struct TaskArguments {
    binds: Option<Ident>,
    capacity: Option<usize>,
    priority: usize,
    shared: (Vec<Stmt>, TokenStream),

    local: (Vec<Stmt>, TokenStream),
    period: Option<(u32, u32)>,
    deadline: Option<(u32, u32)>,
}

impl TaskArguments {
    // NOTE: This assumes that the target is rtic 1 for now.
    fn to_symex_init(&self, input_task_name: Ident) -> (TokenStream, Ident) {
        let task_name = self.binds.clone().unwrap_or(input_task_name.clone());
        let shared = self
            .shared
            .0
            .iter()
            .map(|el| match el {
                Stmt::Ident(i) => i,
                Stmt::Assign(i, _, _) => i,
                Stmt::LockFree(i) => i,
            })
            .map(|el| {
                let id =
                    syn::Ident::new(&format!("__rtic_internal_shared_resource_{el}"), el.span());
                quote! {#id}
            })
            .collect::<Vec<_>>();
        let local = self
            .local
            .0
            .iter()
            .map(|el| match el {
                Stmt::Ident(i) => syn::Ident::new(&format!("resource_{i}"), i.span()),
                Stmt::Assign(i, _, _) => {
                    syn::Ident::new(&format!("{input_task_name}_{i}"), i.span())
                }
                Stmt::LockFree(i) => syn::Ident::new(&format!("resource_{i}"), i.span()),
            })
            .map(|el| {
                let id = syn::Ident::new(&format!("__rtic_internal_local_{el}"), el.span());
                quote! {#id}
            })
            .collect::<Vec<_>>();
        let prio = self.priority;
        let prio_setter = quote! {symex_lib::set_priority(unsafe{core::mem::transmute(#task_name as *mut ())},#prio as u32)};
        let analyze_setter =
            quote! {symex_lib::analyze(unsafe{core::mem::transmute(#task_name as *mut ())})};
        let period_setter = match self.period {
            Some((nom, denom)) => {
                quote! {symex_lib::set_period(unsafe{core::mem::transmute(#task_name as *mut ())},#nom,#denom)}
            }
            _ => quote! {},
        };
        let deadline_setter = match self.deadline {
            Some((nom, denom)) => {
                quote! {symex_lib::set_deadline(unsafe{core::mem::transmute(#task_name as *mut ())},#nom,#denom)}
            }
            _ => quote! {},
        };

        let layout_getters = local.iter().chain(shared.iter()).map(|el| {
            quote! {
                symex_lib::grant_access(unsafe{core::mem::transmute(#task_name as *mut ())},&#el);
            }
        }).collect::<Vec<_>>();

        let id = syn::Ident::new(&format!("__symex_init_{task_name}"), task_name.span());
        let init_name = {
            quote! {#id}
        };
        //let init_name_str = init_name.to_string();
        let init_name_static = Ident::new(&init_name.to_string().to_uppercase(), task_name.span());

        let ret = quote! {
            #[inline(never)]
            #[no_mangle]
            #[allow(non_snake_case)]
            //#[link_name = #init_name_str]
            #[link_section = ".text.symex"]
            /// Initiates the Symex tool for auto discovery of
            unsafe extern "C" fn #init_name () {
                symex_lib::if_should_analyse!{
                    #(#layout_getters)*
                    #prio_setter;
                    #analyze_setter;
                    #period_setter;
                    #deadline_setter;
                    core::hint::black_box(#init_name);
                };
            }


        };
        println!("{ret}");
        (ret.into(), id)
    }
}

impl Parse for TaskArguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![#]) {
            let _: Token![#] = input.parse()?;
            let content;
            bracketed!(content in input);

            let _: Ident = content.parse()?;
            let inner;
            parenthesized!(inner in content);
            let ret = inner.parse();

            println!("inner {inner}");
            return ret;
        }
        let mut buffer = TaskArguments::default();

        println!("Buffer : {:?}", input);

        while input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let _: Token!(=) = input.parse()?;
            match ident.to_string().as_str() {
                "shared" => {
                    let content;
                    bracketed!(content in input);
                    let content_text: proc_macro2::TokenStream = content.fork().parse()?;
                    println!("Content: {content_text}");

                    let shared = (Punctuated::<Stmt, Token![,]>::parse_terminated(&content)?)
                        .into_iter()
                        .collect::<Vec<Stmt>>();
                    buffer.shared = (shared, content_text.into());
                }
                "local" => {
                    let content;
                    bracketed!(content in input);
                    let content_text: proc_macro2::TokenStream = content.fork().parse()?;

                    let local = (Punctuated::<Stmt, Token![,]>::parse_terminated(&content)?)
                        .into_iter()
                        .collect::<Vec<Stmt>>();
                    buffer.local = (local, content_text.into());
                }

                "capacity" => {
                    buffer.capacity = Some(parse_usize(&input)?);
                }

                "priority" => {
                    buffer.priority = parse_usize(&input)?;
                }
                "binds" => buffer.binds = Some(input.parse()?),
                "period" => {
                    buffer.period = {
                        let nom = parse_usize(&input)? as u32;
                        let denom = match input.peek(Token![/]) {
                            true => {
                                let _: Token!(/) = input.parse()?;
                                parse_usize(&input)? as u32
                            }
                            _ => 1,
                        };
                        Some((nom, denom))
                    }
                }
                "deadline" => {
                    buffer.deadline = {
                        let nom = parse_usize(&input)? as u32;
                        let denom = match input.peek(Token![/]) {
                            true => {
                                let _: Token!(/) = input.parse()?;
                                parse_usize(&input)? as u32
                            }
                            _ => 1,
                        };
                        Some((nom, denom))
                    }
                }

                value => {
                    println!("Found {value}");
                    todo!("All task args")
                }
            }
            if input.peek2(Ident) {
                println!("{input}");
                let _: syn::Result<Token![,]> = input.parse();
            } else if !input.is_empty() {
                println!("{input}");
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(buffer)
    }
}

fn parse_usize(input: &syn::parse::ParseStream) -> syn::Result<usize> {
    let val = input.parse::<LitInt>()?;
    usize::from_str_radix(&val.to_string(), 10)
        .map_err(|_| syn::Error::new_spanned(val, "Could not parse as u32"))
}

#[proc_macro_attribute]
pub fn task_easy(input: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as TaskArguments);

    let item: proc_macro2::TokenStream = item.into();
    let shared: proc_macro2::TokenStream = input.shared.1.into();
    let local: proc_macro2::TokenStream = input.local.1.into();
    let binds: proc_macro2::TokenStream = input.binds.map_or(quote!(), |val| quote!(binds = #val,));
    let capacity = input.capacity.map_or(quote!(), |val| {
        let val = LitInt::new(&val.to_string(), Span::call_site());
        quote!(capacity = #val,)
    });
    let priority = input.priority;
    let priority = LitInt::new(&priority.to_string(), Span::call_site());
    let priority = quote! {priority = #priority};

    let output = quote! {
        #[task(#binds local = [#local], shared = [#shared], #capacity #priority)]
        #item
    };
    println!("{output}");
    output.into()
}

fn _task_easy(
    input: TokenStream,
    item: TokenStream,
) -> Result<(TokenStream, TaskArguments), syn::Error> {
    // Parse the input tokens into a syntax tree.
    let input: TaskArguments = match syn::parse(input) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let item: proc_macro2::TokenStream = item.into();
    let shared: proc_macro2::TokenStream = input.shared.1.clone().into();
    let shared = match shared.is_empty() {
        true => proc_macro2::TokenStream::new(),
        _ => quote! {shared = [#shared],},
    };
    let local: proc_macro2::TokenStream = input.local.1.clone().into();
    let local = match local.is_empty() {
        true => proc_macro2::TokenStream::new(),
        _ => quote! {local = [#local],},
    };
    let binds: proc_macro2::TokenStream = input
        .binds
        .clone()
        .map_or(quote!(), |val| quote!(binds = #val,));
    let capacity = input.capacity.map_or(quote!(), |val| {
        let val = LitInt::new(&val.to_string(), Span::call_site());
        quote!(capacity = #val,)
    });
    let priority = input.priority;
    let priority = LitInt::new(&priority.to_string(), Span::call_site());
    let priority = quote! {priority = #priority};

    let output = quote! {
        #[task(#binds #local #shared #capacity #priority)]
        #item
    };
    println!("{output}");
    Ok((output.into(), input))
}

struct AttrWrapper {
    attr: Attribute,
}
impl Parse for AttrWrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attr: Attribute::parse_outer(input)?[0].clone(),
        })
    }
}

// TODO: Add fields to this to allow memory map linking etc
struct Translator(proc_macro2::TokenStream);

impl Parse for Translator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _input: Token![#] = input.parse()?;
        let content;
        bracketed!(content in input);
        let _ident: Ident = content.parse()?;

        let tokens: proc_macro2::TokenStream = content.parse()?;
        Ok(Translator(tokens))
    }
}

impl Translator {
    fn to_rtic(self) -> proc_macro2::TokenStream {
        let fields = self.0;
        quote! {#[rtic::app #fields]}
    }
}

#[proc_macro_attribute]
pub fn easy(input: TokenStream, item: TokenStream) -> TokenStream {
    println!("{input}");
    println!("{item}");
    let mut module = parse_macro_input!(item as ItemMod);
    println!("Zooom");
    for idx in (0..(module.attrs.len())).rev() {
        let attr = module.attrs[idx].clone();
        if attr
            .path()
            .segments
            .last()
            .map(|el| el.ident.to_string())
            .unwrap_or("".to_string())
            .eq("easy")
        {
            module.attrs.remove(idx);
            let translate: Translator = syn::parse2(attr.to_token_stream()).unwrap();
            let rtic = translate.to_rtic();
            let new_attr: AttrWrapper = syn::parse2(rtic).unwrap();
            let new_attr: Attribute = new_attr.attr;
            module.attrs.push(new_attr);
            //attr.
        }
    }
    println!("Zooom2");
    let content: proc_macro2::TokenStream = input.into();
    let rtic = Translator(quote! {(#content)}).to_rtic();
    println!("Zooom3 {rtic}");
    let new_attr: AttrWrapper = syn::parse2(rtic).unwrap();
    println!("Zooom3");
    let new_attr: Attribute = new_attr.attr;
    module.attrs.push(new_attr);
    module.attrs.retain(|segment| {
        !segment
            .path()
            .segments
            .last()
            .map(|el| el.ident.to_string())
            .unwrap_or("".to_string())
            .eq("easy")
    });
    //module.attrs.push(value);
    if module.content.is_none() {
        return quote! {compile_error!("easy cannot be applied to empty modules.")}.into();
    }

    // Safety: This was just checked.
    let (_, content) = unsafe { module.content.as_mut().unwrap_unchecked() };
    let mut new_functions: Vec<ItemFn> = Vec::new();
    let mut new_statics: Vec<ItemStatic> = Vec::new();
    let mut functions_must_call = Vec::new();
    let mut init: Option<&mut ItemFn> = None;
    for item in content.iter_mut().filter(|el| is_fn(el)).map(get_fn) {
        let mut to_add = Vec::new();
        for idx in (0..(item.attrs.len())).rev() {
            let attr = item.attrs[idx].clone();
            let name = attr.path().segments.last().unwrap().ident.to_string();
            if &name == "task" {
                println!("Function {} IDX {idx}", item.sig.ident);
                println!("{}", attr.to_token_stream());
                let (tokens, args) =
                    _task_easy(attr.to_token_stream().into(), TokenStream::new()).unwrap();
                let attr: AttrWrapper = syn::parse(tokens).unwrap();
                to_add.push(attr.attr);
                item.attrs.remove(idx);
                let name = item.sig.ident.clone();
                let (def, id) = args.to_symex_init(name);

                new_functions.push(syn::parse(def).unwrap());
                //new_statics.push(syn::parse(force_used).unwrap());
                functions_must_call.push(id);
            }
        }

        item.attrs.extend_from_slice(&to_add);
        if &item.sig.ident.to_string() == "init" {
            init = Some(item)
        }
    }
    if let Some(init) = init {
        let mut new_stamements: Vec<_> = functions_must_call
            .iter()
            .map(|el| {
                syn::parse2(quote! {
                        symex_lib::if_should_analyse!{
                            unsafe {#el ()};
                        };
                })
                .unwrap()
            })
            .collect();
        new_stamements.extend_from_slice(&init.block.stmts);
        init.block.stmts = new_stamements;
    }

    content.extend(new_functions.iter().map(|el| Item::Fn(el.clone())));
    content.extend(new_statics.iter().map(|el| Item::Static(el.clone())));
    let ret = module.to_token_stream();

    println!("Post init {ret}");
    ret.into()
}

fn is_fn(item: &Item) -> bool {
    match item {
        Item::Fn(_) => true,
        _ => false,
    }
}
fn get_fn(item: &mut Item) -> &mut ItemFn {
    match item {
        Item::Fn(ref mut el) => el,
        _ => unsafe { unreachable_unchecked() },
    }
}

fn is_easy_task(attr: &Attribute) -> bool {
    attr.path()
        .segments
        .first()
        .map(|s| s.ident == "")
        .unwrap_or(false)
}
