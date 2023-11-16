use std::{fmt::Display, marker::PhantomData};

use oxc_ast::{ast::*, Visit};

use oxc_span::Atom;

pub struct ParseExports<'a> {
    lifetime: PhantomData<&'a ()>,
    exports: Vec<Export>,
}

impl<'a> ParseExports<'a> {
    pub fn new() -> Self {
        Self {
            lifetime: PhantomData,
            exports: Vec::new(),
        }
    }

    pub fn parse(mut self, input: &'a Program<'a>) -> Vec<Export> {
        self.visit_program(input);
        self.exports
    }
}

pub enum Export {
    Name(Atom),
    Wildcard,
    Default,
}

impl Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(atom) => atom.fmt(f),
            Self::Wildcard => f.write_str("*"),
            Self::Default => f.write_str("default"),
        }
    }
}

impl<'a> Visit<'a> for ParseExports<'a> {
    fn visit_export_named_declaration(&mut self, decl: &'a ExportNamedDeclaration<'a>) {
        if decl.declaration.is_some() && !decl.specifiers.is_empty() {
            todo!("conflict 1");
        }
        if let Some(ref it) = decl.declaration {
            match it {
                Declaration::VariableDeclaration(it) => {
                    for name in names_of_variable_decl(it) {
                        self.exports.push(Export::Name(name));
                    }
                }
                it => {
                    if let Some(name) = name_of_single_decl(it) {
                        self.exports.push(Export::Name(name));
                    }
                }
            }
        }
        for spec in decl.specifiers.iter() {
            let name = spec.exported.name().clone();
            self.exports.push(Export::Name(name));
        }
    }

    fn visit_export_default_declaration(&mut self, _decl: &'a ExportDefaultDeclaration<'a>) {
        self.exports.push(Export::Default);
    }

    fn visit_export_all_declaration(&mut self, _decl: &'a ExportAllDeclaration<'a>) {
        self.exports.push(Export::Wildcard);
    }
}

pub fn name_of_single_decl(it: &Declaration) -> Option<Atom> {
    match it {
        Declaration::FunctionDeclaration(it) => name_of_some_binding(it.id.as_ref()),
        Declaration::ClassDeclaration(it) => name_of_some_binding(it.id.as_ref()),
        Declaration::TSTypeAliasDeclaration(it) => Some(it.id.name.clone()),
        Declaration::TSInterfaceDeclaration(it) => Some(it.id.name.clone()),
        Declaration::TSEnumDeclaration(it) => Some(it.id.name.clone()),
        Declaration::TSModuleDeclaration(it) => Some(it.id.name().clone()),
        Declaration::TSImportEqualsDeclaration(it) => Some(it.id.name.clone()),
        Declaration::VariableDeclaration(_) => unimplemented!("use names_of_variable_decl"),
    }
}

fn name_of_some_binding(it: Option<&BindingIdentifier>) -> Option<Atom> {
    it.map(|it| it.name.clone())
}

fn names_of_variable_decl(it: &VariableDeclaration) -> Vec<Atom> {
    it.declarations
        .iter()
        .flat_map(|it| names_of_binding_pattern_kind(&it.id.kind))
        .collect()
}

fn names_of_binding_pattern_kind(it: &BindingPatternKind) -> Vec<Atom> {
    match it {
        BindingPatternKind::BindingIdentifier(it) => vec![it.name.clone()],
        BindingPatternKind::ObjectPattern(it) => it
            .properties
            .iter()
            .flat_map(|it| names_of_binding_pattern_kind(&it.value.kind))
            .collect(),
        BindingPatternKind::ArrayPattern(it) => it
            .elements
            .iter()
            .flatten()
            .flat_map(|it| names_of_binding_pattern_kind(&it.kind))
            .collect(),
        BindingPatternKind::AssignmentPattern(it) => names_of_binding_pattern_kind(&it.left.kind),
    }
}
