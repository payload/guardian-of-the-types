use std::{env, path::Path};

use oxc_allocator::Allocator;

use oxc_parser::Parser;
use oxc_span::SourceType;

mod guardians;
mod parse_exports;

fn main() {
    let name = env::args()
        .nth(1)
        .unwrap_or_else(|| "test/test.ts".to_string());
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("{name} not found"));
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).unwrap();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
        return;
    }

    let program = allocator.alloc(ret.program);
    let exports = parse_exports::ParseExports::new().parse(program);

    println!(
        "export {{ {} }} from \"test/test.ts\"",
        exports
            .into_iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let guardians = guardians::Guardians::new().parse(program);
    println!("\nGuardians:\n");
    for guardian in guardians {
        println!("{guardian}");
    }
}
