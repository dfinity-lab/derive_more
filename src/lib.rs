#![feature(box_syntax, rustc_private, quote)]

#[macro_use]
extern crate rustc;
extern crate proc_macro;
extern crate rustc_plugin;
#[macro_use]
extern crate syntax;
extern crate syntax_ext;
#[macro_use]
extern crate quote;
extern crate syn;

use rustc_plugin::Registry;
use syntax::symbol::Symbol;
use syntax::ext::base::MultiDecorator;
use proc_macro::TokenStream;

mod from;
mod from_new;
mod add_like;
mod mul_like;

const ADDLIKE_OPS: &'static [&'static str] = &["Add", "Sub", "BitAnd", "BitOr", "BitXor"];
const MULLIKE_OPS: &'static [&'static str] = &["Mul", "Div", "Rem", "Shr", "Shl"];

#[proc_macro_derive(From)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    from_new::expand(&ast).parse().unwrap()
}

//pub fn plugin_registrar(reg: &mut Registry) {
//    reg.register_syntax_extension(Symbol::intern("derive_From"), MultiDecorator(box from::expand));
//    for op in ADDLIKE_OPS {
//        let expand = move |cx: &mut ExtCtxt, span: Span, _: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)| {
//            add_like::expand(cx, span, item, push, op)
//        };
//        reg.register_syntax_extension(Symbol::intern(&format!("derive_{}", op)), MultiDecorator(box expand));
//    }
//    for op in MULLIKE_OPS {
//        let expand = move |cx: &mut ExtCtxt, span: Span, _: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)| {
//            mul_like::expand(cx, span, item, push, op)
//        };
//        reg.register_syntax_extension(Symbol::intern(&format!("derive_{}", op)), MultiDecorator(box expand));
//    }
//}


use syntax::ast::*;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};