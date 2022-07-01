use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{GenericArgument, GenericParam, Generics, Path, Token, Type, TypePath};

/// Alias for a comma-separated list of `GenericArgument`
pub type GenericArguments = Punctuated<GenericArgument, Token![,]>;

/// Finds the index of the first generic param with a default value or a const generic.
fn first_default_or_const_param_position(generics: &Generics) -> Option<usize> {
    generics.params.iter().position(|param| match param {
        GenericParam::Type(param) => param.default.is_some(),
        GenericParam::Const(_) => true,
        _ => false,
    })
}

/// Push a type GenericParam into a Generics
pub fn push_type_param(generics: &mut Generics, type_param: GenericParam) {
    if let Some(idx) = first_default_or_const_param_position(generics) {
        generics.params.insert(idx, type_param)
    } else {
        generics.params.push(type_param)
    }
}

/// Converts `GenericParams` into `GenericArguments`.
pub fn to_arguments(generics: &Generics) -> GenericArguments {
    let mut args: GenericArguments = Punctuated::new();
    args.extend(generics.params.iter().map(|param| match param {
        GenericParam::Type(type_param) => new_generic_type_arg(type_param.ident.clone()),
        GenericParam::Lifetime(lifetime_param) => {
            GenericArgument::Lifetime(lifetime_param.lifetime.clone())
        }
        GenericParam::Const(const_param) => new_generic_type_arg(const_param.ident.clone()),
    }));
    args
}

// Creates a `GenericArgument` from an `Ident`
fn new_generic_type_arg(ident: Ident) -> GenericArgument {
    GenericArgument::Type(Type::Path(TypePath {
        path: Path::from(ident),
        qself: None,
    }))
}
