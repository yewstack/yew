use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, token::Colon2, GenericArgument, GenericParam, Generics, Path,
    PathArguments, PathSegment, Token, TraitBound, TraitBoundModifier, Type, TypeParam,
    TypeParamBound, TypePath,
};

/// Alias for a comma-separated list of `GenericArgument`
pub type GenericArguments = Punctuated<GenericArgument, Token![,]>;

/// Finds the index of the first generic param with a default value.
fn first_default_param_position(generics: &Generics) -> Option<usize> {
    generics.params.iter().position(|param| match param {
        GenericParam::Type(param) => param.default.is_some(),
        _ => false,
    })
}

/// Converts `GenericParams` into `GenericArguments` and adds `type_ident` as a type arg.
/// `type_ident` is added at the end of the existing type arguments which don't have a default value.
pub fn to_arguments(generics: &Generics, type_ident: Ident) -> GenericArguments {
    let mut args: GenericArguments = Punctuated::new();
    args.extend(generics.params.iter().map(|param| match param {
        GenericParam::Type(type_param) => new_generic_type_arg(type_param.ident.clone()),
        GenericParam::Lifetime(lifetime_param) => {
            GenericArgument::Lifetime(lifetime_param.lifetime.clone())
        }
        _ => unimplemented!("const params are not supported in the derive macro"),
    }));

    let new_arg = new_generic_type_arg(type_ident);
    if let Some(index) = first_default_param_position(generics) {
        args.insert(index, new_arg);
    } else {
        args.push(new_arg);
    }

    args
}

/// Adds a new bounded `GenericParam` to a `Generics`
/// The new param is added after the existing ones without a default value.
pub fn with_param_bounds(generics: &Generics, param_ident: Ident, param_bounds: Ident) -> Generics {
    let mut new_generics = generics.clone();
    let params = &mut new_generics.params;
    let new_param = new_param_bounds(param_ident, param_bounds);
    if let Some(index) = first_default_param_position(generics) {
        params.insert(index, new_param);
    } else {
        params.push(new_param);
    }

    new_generics
}

// Creates a `GenericArgument` from an `Ident`
fn new_generic_type_arg(ident: Ident) -> GenericArgument {
    GenericArgument::Type(Type::Path(TypePath {
        path: Path::from(ident),
        qself: None,
    }))
}

// Creates a bounded `GenericParam` from two `Ident`
fn new_param_bounds(param_ident: Ident, param_bounds: Ident) -> GenericParam {
    let mut path_segments: Punctuated<PathSegment, Colon2> = Punctuated::new();
    path_segments.push(PathSegment {
        ident: param_bounds,
        arguments: PathArguments::None,
    });

    let mut param_bounds: Punctuated<TypeParamBound, Token![+]> = Punctuated::new();
    param_bounds.push(TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: Path {
            leading_colon: None,
            segments: path_segments,
        },
    }));

    GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: param_ident,
        colon_token: Some(Token![:](Span::call_site())),
        bounds: param_bounds,
        eq_token: None,
        default: None,
    })
}
