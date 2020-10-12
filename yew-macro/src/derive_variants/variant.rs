use proc_macro2::{Ident};
use std::convert::TryFrom;
use syn::parse::Result;
use syn::{Error, Fields, Type, Variant};

pub struct VariantsVariant {
    pub ty: Type,
    pub name: Ident,
}

impl TryFrom<Variant> for VariantsVariant {
    type Error = Error;

    fn try_from(variant: Variant) -> Result<Self> {
        let fields = match variant.fields {
            Fields::Unnamed(fields) => fields.unnamed,
            _ => unimplemented!("only unnamed fields are supported"),
        };
        if fields.len() > 1 { unimplemented!("only unnamed fields with a single field are supported"); }
        let field = fields.first().unwrap();
        Ok(VariantsVariant {
            ty: field.ty.clone(),
            name: variant.ident,
        })
    }
}

