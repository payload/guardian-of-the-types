use std::marker::PhantomData;

use oxc_ast::{ast::*, Visit};

use crate::parse_exports::{self, name_of_single_decl};

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
        let Some(ref it) = decl.declaration else {
            return;
        };
        match it {
            Declaration::TSTypeAliasDeclaration(it) => self.guardians.push(self.guard_type(it)),
            Declaration::TSInterfaceDeclaration(it) => {
                self.guardians.push(self.guard_interface(it))
            }
            Declaration::VariableDeclaration(_) => todo!(),
            Declaration::FunctionDeclaration(_) => todo!(),
            Declaration::ClassDeclaration(_) => todo!(),
            Declaration::TSEnumDeclaration(_) => todo!(),
            Declaration::TSModuleDeclaration(_) => todo!(),
            Declaration::TSImportEqualsDeclaration(_) => todo!(),
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
    fn guard_interface(&self, it: &TSInterfaceDeclaration) -> Guardian {
        let name = it.id.name.clone();
        let body = &it.body.body;
        let checks: Vec<_> = body
            .iter()
            .map(|it| it.check_code("it"))
            .collect();
        let check_code = format!(
            "it && typeof it === 'object' &&\n\t{}",
            checks.join(" &&\n\t")
        );
        Guardian {
            typename: name.to_string(),
            check_code,
        }
    }

    fn guard_type(&self, it: &TSTypeAliasDeclaration) -> Guardian {
        let name = it.id.name.clone();
        match &it.type_annotation {
            TSType::TSUnionType(it) => Guardian::new(&name, it.check_code("it")),
            _ => todo!(),
        }
    }
}

trait CheckCode {
    fn check_code(&self, left: &str) -> String;
}

impl<'a> CheckCode for TSIndexSignature<'a> {
    fn check_code(&self, left: &str) -> String {
        assert_eq!(self.parameters.len(), 1);
        let p = &self.parameters[0];
        let pname = &p.name;
        let paccessor = format!("{left}[{pname}]");
        let pcheck = p.type_annotation.type_annotation.check_code(&paccessor);
        format!("Object.entries({left}).reduce((a, ([{pname}, v]) => ({pcheck}), true)")
    }
}

impl<'a> CheckCode for TSPropertySignature<'a> {
    fn check_code(&self, left: &str) -> String {
        let key = match &self.key {
            PropertyKey::Identifier(it) => it.name.as_str(),
            PropertyKey::PrivateIdentifier(_) => todo!(),
            PropertyKey::Expression(_) => todo!(),
        };
        let Some(it) = &self.type_annotation else {
            todo!()
        };
        match &it.type_annotation {
            TSType::TSAnyKeyword(_) => format!("'{key}' in {left}"),
            it => it.check_code(&format!("{left}.{key}")),
        }
    }
}

impl<'a> CheckCode for TSUnionType<'a> {
    fn check_code(&self, left: &str) -> String {
        let checks = self
            .types
            .iter()
            .map(|t| t.check_code(left))
            .collect::<Vec<_>>();
        if checks.len() > 1 {
            format!("({})", checks.join(" || "))
        } else {
            checks.join("")
        }
    }
}

impl<'a> CheckCode for TSType<'a> {
    fn check_code(&self, left: &str) -> String {
        match self {
            TSType::TSAnyKeyword(_) => todo!(),
            TSType::TSBigIntKeyword(_) => todo!(),
            TSType::TSBooleanKeyword(_) => todo!(),
            TSType::TSNeverKeyword(_) => todo!(),
            TSType::TSNullKeyword(_) => todo!(),
            TSType::TSNumberKeyword(_) => format!("typeof {left} === 'number'"),
            TSType::TSObjectKeyword(_) => format!("typeof {left} === 'object'"),
            TSType::TSStringKeyword(_) => format!("typeof {left} === 'string'"),
            TSType::TSSymbolKeyword(_) => todo!(),
            TSType::TSThisKeyword(_) => todo!(),
            TSType::TSUndefinedKeyword(_) => todo!(),
            TSType::TSUnknownKeyword(_) => todo!(),
            TSType::TSVoidKeyword(_) => todo!(),
            TSType::TSArrayType(it) => it.check_code(left),
            TSType::TSConditionalType(_) => todo!(),
            TSType::TSConstructorType(_) => todo!(),
            TSType::TSFunctionType(_) => todo!(),
            TSType::TSImportType(_) => todo!(),
            TSType::TSIndexedAccessType(_) => todo!(),
            TSType::TSInferType(_) => todo!(),
            TSType::TSIntersectionType(_) => todo!(),
            TSType::TSLiteralType(it) => it.literal.check_code(left),
            TSType::TSMappedType(_) => todo!(),
            TSType::TSQualifiedName(_) => todo!(),
            TSType::TSTemplateLiteralType(_) => todo!(),
            TSType::TSTupleType(_) => todo!(),
            TSType::TSTypeLiteral(it) => it.check_code(left),
            TSType::TSTypeOperatorType(_) => todo!(),
            TSType::TSTypePredicate(_) => todo!(),
            TSType::TSTypeQuery(_) => todo!(),
            TSType::TSTypeReference(it) => match &it.type_name {
                TSTypeName::IdentifierReference(it) => format!("is{}({left})", it.name),
                TSTypeName::QualifiedName(_) => todo!(),
            },
            TSType::TSUnionType(it) => it.check_code(left),
            TSType::JSDocNullableType(_) => todo!(),
            TSType::JSDocUnknownType(_) => todo!(),
        }
    }
}

impl<'a> CheckCode for TSTypeLiteral<'a> {
    fn check_code(&self, left: &str) -> String {
        let checks: Vec<_> = self.members.iter().map(|it| it.check_code(left)).collect();
        checks.join(" : ")
    }
}

impl<'a> CheckCode for TSSignature<'a> {
    fn check_code(&self, left: &str) -> String {
        match self {
            TSSignature::TSIndexSignature(it) => it.check_code(left),
            TSSignature::TSPropertySignature(it) => it.check_code(left),
            TSSignature::TSCallSignatureDeclaration(_) => todo!(),
            TSSignature::TSConstructSignatureDeclaration(_) => todo!(),
            TSSignature::TSMethodSignature(_) => todo!(),
        }
    }
}

impl<'a> CheckCode for TSArrayType<'a> {
    fn check_code(&self, left: &str) -> String {
        let check_element = self.element_type.check_code("b");
        format!("(Array.isArray({left}) && {left}.reduce((a, b) => ({check_element}), true))")
    }
}

impl<'a> CheckCode for TSLiteral<'a> {
    fn check_code(&self, left: &str) -> String {
        match self {
            TSLiteral::BooleanLiteral(it) => {
                if it.value {
                    format!("{left} === true")
                } else {
                    format!("{left} === false")
                }
            }
            TSLiteral::NullLiteral(_) => todo!(),
            TSLiteral::NumberLiteral(_) => todo!(),
            TSLiteral::BigintLiteral(_) => todo!(),
            TSLiteral::RegExpLiteral(_) => todo!(),
            TSLiteral::StringLiteral(it) => format!("{left} === '{}'", it.value),
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
        if let Some((name, _)) = debug.split_once([' ', '(', '{', '[']) {
            name.to_string()
        } else {
            debug
        }
    }
}
