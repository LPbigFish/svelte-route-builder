use std::path::Path;

use swc_common::sync::Lrc;
use swc_common::{Spanned, DUMMY_SP};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::{Decl, ExportDecl, Expr, Lit, Module, Number, VarDecl, VarDeclarator};
use swc_ecma_codegen::text_writer::JsWriter;
use swc_ecma_codegen::Emitter;
use swc_ecma_parser::TsConfig;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{Fold, FoldWith};

const PAGE_SERVER_NAMES: [&str; 2] = ["server_load", "actions"];
const SERVER_NAMES: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];
const CLIENT_NAMES: [&str; 1] = ["client_load"];

#[derive(Debug)]
struct FunctionFolder {
    page_server: Vec<VarDeclarator>,
    server: Vec<VarDeclarator>,
    client: Vec<VarDeclarator>,
}

impl Fold for FunctionFolder {
    fn fold_export_decl(&mut self,n:swc_ecma_ast::ExportDecl) -> swc_ecma_ast::ExportDecl {
        let decls = Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: n.decl.as_var().expect("Oops, no var here").kind,
            declare: n.decl.as_var().expect("Oops, no declare var here too").declare,
            decls: n.decl.as_var().expect("Oops haha, nothing here too").decls.clone().into_iter().map(|mut decl| {
                let mut which_array = &mut self.server;
                if let Some(ident) = decl.name.as_mut_ident() {
                    let mut new_decl_name = ident.clone();
                    new_decl_name.span = DUMMY_SP;
                    if CLIENT_NAMES.contains(&ident.sym.as_str()) {
                        which_array = &mut self.client;
                        new_decl_name.sym = format!("{}",ident.sym.replace("client_", "")).into();
                    } else if PAGE_SERVER_NAMES.contains(&ident.sym.as_str()) {
                        which_array = &mut self.page_server;
                        new_decl_name.sym = format!("{}",ident.sym.replace("server_", "")).into();
                    } else if SERVER_NAMES.contains(&ident.sym.as_str()) {
                        new_decl_name.sym = format!("{}",ident.sym.replace("server_", "")).into();
                    }

                    let new_var_decl = VarDeclarator {
                        span: DUMMY_SP,
                        name: swc_ecma_ast::Pat::Ident(new_decl_name),
                        init: decl.init.take().map(|init| Box::new(Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: init.span_hi().0 as f64,
                            raw: No,
                        })))),
                        definite: decl.definite,
                    };

                    which_array.push(new_var_decl.clone());

                    decl = new_var_decl;
                }


                decl
            }).collect()}));

        let new_export = ExportDecl {
            span: n.span,
            decl: decls,
        };

        new_export
    }
}

pub fn modulate(path: &str) -> (Module, String) {
    let cm: Lrc<SourceMap> = Default::default();

    let handler = Handler::with_tty_emitter(ColorConfig::Always, true, false, Some(cm.clone()));

    let fm = cm.load_file(Path::new(&path)).expect("Failed to load file");

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        println!("{:?}", e);
        e.into_diagnostic(&handler).emit();
    }

    let mut _module: swc_ecma_ast::Module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module");

    let mut function_folder = FunctionFolder { client: Vec::new(), page_server: Vec::new(), server: Vec::new() };
    let _module = _module.fold_with(&mut function_folder);

    println!("{:?} \n\n {:?} \n\n {:?}", function_folder.client, function_folder.page_server, function_folder.server);

    (_module, path.to_string())    
}

pub fn generate(program: Module, path: String) -> Result<String, std::io::Error> {
    let cm: Lrc<SourceMap> = Default::default();
    cm.load_file(Path::new(&path)).expect("Failed to load file");

    let mut buf = vec![];

    let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);

    let mut emitter = Emitter {
        cfg: swc_ecma_codegen::Config::default(),
        comments: None,
        cm: cm.clone(),
        wr,
    };

    emitter.emit_module(&program)?;

    println!("{}", String::from_utf8(buf.clone()).unwrap());

    Ok(String::from_utf8(buf).unwrap())
}