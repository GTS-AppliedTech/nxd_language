use crate::ir::nodes::{IRImpl, IRFunction};
use crate::backend::nim::functions::emit_function;

pub fn emit_impls(impls: &Vec<IRImpl>) -> String {
    let mut out = String::new();

    for im in impls {
        out.push_str(&emit_impl(im));
        out.push('\n');
    }

    out
}

fn emit_impl(im: &IRImpl) -> String {
    let mut out = String::new();

    // Each method in IMPL becomes a proc with the receiver type
    for method in &im.methods {
        out.push_str(&emit_impl_method(&im.target_type, method));
        out.push('\n');
    }

    out
}

fn emit_impl_method(target_type: &str, func: &IRFunction) -> String {
    let mut f = func.clone();

    // Insert receiver parameter as first argument
    f.params.insert(
        0,
        crate::ir::nodes::IRParam {
            name: "self".to_string(),
            ty: target_type.to_string(),
        },
    );

    emit_function(&f)
}
