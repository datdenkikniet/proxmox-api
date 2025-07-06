use oxc_allocator::Allocator;
use oxc_ast::ast::{BindingPatternKind, Declaration, Statement};
use oxc_parser::Parser;
use oxc_span::{GetSpan, SourceType};

pub fn extract_api_schema(js_content: &str) -> Option<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path("api.js").unwrap();
    let ret = Parser::new(&allocator, js_content, source_type).parse();

    for stmt in ret.program.body {
        if let Statement::Declaration(Declaration::VariableDeclaration(var_decl)) = stmt {
            if let Some(decl) = var_decl.declarations.get(0) {
                if let BindingPatternKind::BindingIdentifier(ident) = &decl.id.kind {
                    if ident.name == "apiSchema" {
                        if let Some(init) = &decl.init {
                            let span = init.span();
                            let schema_text = &js_content[span.start as usize..span.end as usize];
                            return Some(schema_text.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}
