use std::{fmt::Display, marker::PhantomData};

use oxc_ast::{ast::*, Visit};

use oxc_span::Atom;

use crate::parse_exports;

pub struct Guardians<'a> {
    lifetime: PhantomData<&'a ()>,
    guardians: Vec<Guardian>,
}

impl<'a> Guardians<'a> {
    pub fn new() -> Self {
        Self {
            lifetime: PhantomData,
            guardians: Vec::new(),
        }
    }

    pub fn parse(mut self, input: &'a Program<'a>) -> Vec<Guardian> {
        self.visit_program(input);
        self.guardians
    }
}

#[derive(Debug)]
pub struct Guardian {
    typename: String,
    check_code: String,
}

impl Guardian {
    fn new(typename: &str, check_code: String) -> Self {
        Self {
            typename: typename.into(),
            check_code,
        }
    }
}

impl std::fmt::Display for Guardian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            typename,
            check_code,
        } = &self;
        f.write_fmt(format_args!(
            "\
        function is{typename}(it: any): it is {typename} {{\n\
        \treturn {check_code};\n\
        }}\n
        "
        ))
    }
}

impl<'a> Visit<'a> for Guardians<'a> {
    fn visit_export_named_declaration(&mut self, decl: &'a ExportNamedDeclaration<'a>) {
        if decl.declaration.is_some() && !decl.specifiers.is_empty() {
            todo!("conflict 1");
        }
        if let Some(ref it) = decl.declaration {
            match it {
                Declaration::VariableDeclaration(it) => {
                    todo!();
                }
                it => {
                    if let Some(name) = parse_exports::name_of_single_decl(it) {
                        println!("{name:16} {:32}", it.debug_name());
                        if let Declaration::TSTypeAliasDeclaration(it) = it {
                            let guardian = self.guard_type(&name, &it.type_annotation);
                            self.guardians.push(guardian);
                        }
                    }
                }
            }
        }
        for spec in decl.specifiers.iter() {
            todo!();
        }
    }

    fn visit_export_default_declaration(&mut self, _decl: &'a ExportDefaultDeclaration<'a>) {
        todo!();
    }

    fn visit_export_all_declaration(&mut self, _decl: &'a ExportAllDeclaration<'a>) {
        todo!();
    }
}

impl<'a> Guardians<'a> {
    fn guard_type(&self, name: &str, it: &TSType) -> Guardian {
        match it {
            TSType::TSAnyKeyword(_) => todo!(),
            TSType::TSBigIntKeyword(_) => todo!(),
            TSType::TSBooleanKeyword(_) => todo!(),
            TSType::TSNeverKeyword(_) => todo!(),
            TSType::TSNullKeyword(_) => todo!(),
            TSType::TSNumberKeyword(_) => todo!(),
            TSType::TSObjectKeyword(_) => todo!(),
            TSType::TSStringKeyword(_) => todo!(),
            TSType::TSSymbolKeyword(_) => todo!(),
            TSType::TSThisKeyword(_) => todo!(),
            TSType::TSUndefinedKeyword(_) => todo!(),
            TSType::TSUnknownKeyword(_) => todo!(),
            TSType::TSVoidKeyword(_) => todo!(),
            TSType::TSArrayType(_) => todo!(),
            TSType::TSConditionalType(_) => todo!(),
            TSType::TSConstructorType(_) => todo!(),
            TSType::TSFunctionType(_) => todo!(),
            TSType::TSImportType(_) => todo!(),
            TSType::TSIndexedAccessType(_) => todo!(),
            TSType::TSInferType(_) => todo!(),
            TSType::TSIntersectionType(_) => todo!(),
            TSType::TSLiteralType(_) => todo!(),
            TSType::TSMappedType(_) => todo!(),
            TSType::TSQualifiedName(_) => todo!(),
            TSType::TSTemplateLiteralType(_) => todo!(),
            TSType::TSTupleType(_) => todo!(),
            TSType::TSTypeLiteral(_) => todo!(),
            TSType::TSTypeOperatorType(_) => todo!(),
            TSType::TSTypePredicate(_) => todo!(),
            TSType::TSTypeQuery(_) => todo!(),
            TSType::TSTypeReference(_) => todo!(),
            TSType::TSUnionType(it) => Guardian::new(&name, it.check_code()),
            TSType::JSDocNullableType(_) => todo!(),
            TSType::JSDocUnknownType(_) => todo!(),
        }
    }
}

trait CheckCode {
    fn check_code(&self) -> String;
}

impl<'a> CheckCode for TSUnionType<'a> {
    fn check_code(&self) -> String {
        let checks = self
            .types
            .iter()
            .map(|t| t.check_code())
            .collect::<Vec<_>>();
        checks.join(" || ")
    }
}

impl<'a> CheckCode for TSType<'a> {
    fn check_code(&self) -> String {
        match self {
            TSType::TSAnyKeyword(_) => todo!(),
            TSType::TSBigIntKeyword(_) => todo!(),
            TSType::TSBooleanKeyword(_) => todo!(),
            TSType::TSNeverKeyword(_) => todo!(),
            TSType::TSNullKeyword(_) => todo!(),
            TSType::TSNumberKeyword(_) => todo!(),
            TSType::TSObjectKeyword(_) => todo!(),
            TSType::TSStringKeyword(_) => todo!(),
            TSType::TSSymbolKeyword(_) => todo!(),
            TSType::TSThisKeyword(_) => todo!(),
            TSType::TSUndefinedKeyword(_) => todo!(),
            TSType::TSUnknownKeyword(_) => todo!(),
            TSType::TSVoidKeyword(_) => todo!(),
            TSType::TSArrayType(_) => todo!(),
            TSType::TSConditionalType(_) => todo!(),
            TSType::TSConstructorType(_) => todo!(),
            TSType::TSFunctionType(_) => todo!(),
            TSType::TSImportType(_) => todo!(),
            TSType::TSIndexedAccessType(_) => todo!(),
            TSType::TSInferType(_) => todo!(),
            TSType::TSIntersectionType(_) => todo!(),
            TSType::TSLiteralType(it) => it.literal.check_code(),
            TSType::TSMappedType(_) => todo!(),
            TSType::TSQualifiedName(_) => todo!(),
            TSType::TSTemplateLiteralType(_) => todo!(),
            TSType::TSTupleType(_) => todo!(),
            TSType::TSTypeLiteral(_) => todo!(),
            TSType::TSTypeOperatorType(_) => todo!(),
            TSType::TSTypePredicate(_) => todo!(),
            TSType::TSTypeQuery(_) => todo!(),
            TSType::TSTypeReference(_) => todo!(),
            TSType::TSUnionType(_) => todo!(),
            TSType::JSDocNullableType(_) => todo!(),
            TSType::JSDocUnknownType(_) => todo!(),
        }
    }
}

impl<'a> CheckCode for TSLiteral<'a> {
    fn check_code(&self) -> String {
        match self {
            TSLiteral::BooleanLiteral(it) => {
                if it.value {
                    format!("it === true")
                } else {
                    format!("it === false")
                }
            }
            TSLiteral::NullLiteral(_) => todo!(),
            TSLiteral::NumberLiteral(_) => todo!(),
            TSLiteral::BigintLiteral(_) => todo!(),
            TSLiteral::RegExpLiteral(_) => todo!(),
            TSLiteral::StringLiteral(it) => format!("it === '{}'", it.value),
            TSLiteral::TemplateLiteral(_) => todo!(),
            TSLiteral::UnaryExpression(_) => todo!(),
        }
    }
}

trait DebugName {
    fn debug_name(&self) -> String;
}

impl<T: std::fmt::Debug> DebugName for T {
    fn debug_name(&self) -> String {
        let debug = format!("{self:?}");
        if let Some((name, _)) = debug.split_once(&[' ', '(', '{', '[']) {
            name.to_string()
        } else {
            debug
        }
    }
}
