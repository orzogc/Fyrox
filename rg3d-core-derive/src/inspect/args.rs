//! Derive input types by `darling`.
//!
//! They parse `#[attributes(..)]` in declartive style, different from `syn`.

use darling::*;
use syn::*;

#[derive(FromDeriveInput)]
#[darling(attributes(inspect), supports(struct_any, enum_any))]
pub struct TypeArgs {
    pub ident: Ident,
    pub generics: Generics,
    pub data: ast::Data<VariantArgs, FieldArgs>,
}

/// Parsed from struct's or enum variant's field
#[derive(FromField, Clone)]
#[darling(attributes(inspect))]
pub struct FieldArgs {
    pub ident: Option<Ident>,
    pub ty: Type,
    /// `#[inspect(skip)]`
    ///
    /// Do not expose property info
    #[darling(default)]
    pub skip: bool,
    /// #[inspect(name = "<name>")]
    ///
    /// Name override for a field (default: Title Case)
    #[darling(default)]
    pub name: Option<String>,
    /// #[inspect(group = "<group>")]
    ///
    /// Group override for a field (default: Common)
    #[darling(default)]
    pub group: Option<String>,
    /// `#[inspect(expand)]`
    #[darling(default)]
    pub expand: bool,
}

#[derive(FromVariant)]
#[darling(attributes(inspect))]
pub struct VariantArgs {
    pub ident: Ident,
    pub fields: ast::Fields<FieldArgs>,
}
