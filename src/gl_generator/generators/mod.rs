use registry::*;

mod ty;
pub mod global_gen;
pub mod static_gen;
pub mod struct_gen;

pub fn gen_binding_ident(binding: &Binding, use_idents: bool) -> String {
    // FIXME: use &'a str when https://github.com/mozilla/rust/issues/11869 is
    // fixed
    if use_idents {
        match binding.ident.as_slice() {
            "in" => "in_".to_string(),
            "ref" => "ref_".to_string(),
            "type" => "type_".to_string(),
            ident => ident.to_string(),
        }
    } else {
        "_".to_string()
    }
}

pub fn gen_binding(binding: &Binding, use_idents: bool) -> String {
    format!("{}: {}",
        gen_binding_ident(binding, use_idents),
        ty::to_rust_ty(binding.ty.as_slice()))
}

pub fn gen_param_list(cmd: &Cmd, use_idents: bool) -> String {
    cmd.params.iter()
        .map(|b| gen_binding(b, use_idents))
        .collect::<Vec<String>>()
        .connect(", ")
}

pub fn gen_param_ident_list(cmd: &Cmd) -> String {
    cmd.params.iter()
        .map(|b| gen_binding_ident(b, true))
        .collect::<Vec<String>>()
        .connect(", ")
}

pub fn gen_param_ty_list(cmd: &Cmd) -> String {
    cmd.params.iter()
        .map(|b| ty::to_rust_ty(b.ty.as_slice()))
        .collect::<Vec<&str>>()
        .connect(", ")
}

pub fn gen_return_suffix(cmd: &Cmd) -> String {
    ty::to_return_suffix(ty::to_rust_ty(cmd.proto.ty.as_slice()))
}

pub fn gen_symbol_name(ns: &Ns, cmd: &Cmd) -> String {
    (match *ns {
        Gl | Gles1 | Gles2 => "gl",
        Glx => "glX",
        Wgl => "wgl",
        Egl => "egl",
    }).to_string().append(cmd.proto.ident.as_slice())
}
