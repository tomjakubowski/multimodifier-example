#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::parse::token;

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let nm = token::intern("dare");
    let ext = SyntaxExtension::MultiModifier(Box::new(expand_dare));
    reg.register_syntax_extension(nm, ext);
}

fn expand_dare(excx: &mut ExtCtxt,
                 sp: Span,
                 _: &MetaItem,
                 item: Annotatable) -> Annotatable {
    use syntax::ast::ImplItem_;
    if let Annotatable::ImplItem(ref iitem) = item {
        if let ImplItem_::MethodImplItem(_, _) = iitem.node {
            excx.span_err(sp, "Just Say No to methods!");
        }
    }
    item
}

#[test]
fn it_works() {
}
