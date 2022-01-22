use proc_macro2::Span;
use syn::visit_mut::{self, VisitMut};
use syn::{
    GenericArgument, Lifetime, ParenthesizedGenericArguments, Receiver, TypeImplTrait,
    TypeParamBound, TypeReference,
};

// borrowed from the awesome async-trait crate.
pub struct CollectLifetimes {
    pub elided: Vec<Lifetime>,
    pub explicit: Vec<Lifetime>,
    pub name: &'static str,
    pub default_span: Span,

    pub impl_trait_ctr: u64,
    pub impl_fn_ctr: u64,
}

impl CollectLifetimes {
    pub fn new(name: &'static str, default_span: Span) -> Self {
        CollectLifetimes {
            elided: Vec::new(),
            explicit: Vec::new(),
            name,
            default_span,

            impl_trait_ctr: 0,
            impl_fn_ctr: 0,
        }
    }

    fn is_impl_trait(&self) -> bool {
        self.impl_trait_ctr > 0
    }

    fn is_impl_fn(&self) -> bool {
        self.impl_fn_ctr > 0
    }

    fn visit_opt_lifetime(&mut self, lifetime: &mut Option<Lifetime>) {
        match lifetime {
            None => *lifetime = Some(self.next_lifetime(None)),
            Some(lifetime) => self.visit_lifetime(lifetime),
        }
    }

    fn visit_lifetime(&mut self, lifetime: &mut Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.next_lifetime(lifetime.span());
        } else {
            self.explicit.push(lifetime.clone());
        }
    }

    fn next_lifetime<S: Into<Option<Span>>>(&mut self, span: S) -> Lifetime {
        let name = format!("{}{}", self.name, self.elided.len());
        let span = span.into().unwrap_or(self.default_span);
        let life = Lifetime::new(&name, span);
        self.elided.push(life.clone());
        life
    }
}

impl VisitMut for CollectLifetimes {
    fn visit_receiver_mut(&mut self, arg: &mut Receiver) {
        if let Some((_, lifetime)) = &mut arg.reference {
            self.visit_opt_lifetime(lifetime);
        }
    }

    fn visit_type_reference_mut(&mut self, ty: &mut TypeReference) {
        // We don't rewrite references in the impl FnOnce(&arg)
        if self.is_impl_fn() {
            return;
        }

        self.visit_opt_lifetime(&mut ty.lifetime);
        visit_mut::visit_type_reference_mut(self, ty);
    }

    fn visit_generic_argument_mut(&mut self, gen: &mut GenericArgument) {
        // We don't rewrite types in the impl FnOnce(&arg) -> Type<'_>
        if self.is_impl_fn() {
            return;
        }

        if let GenericArgument::Lifetime(lifetime) = gen {
            self.visit_lifetime(lifetime);
        }
        visit_mut::visit_generic_argument_mut(self, gen);
    }

    fn visit_type_impl_trait_mut(&mut self, impl_trait: &mut TypeImplTrait) {
        self.impl_trait_ctr += 1;

        impl_trait
            .bounds
            .insert(0, TypeParamBound::Lifetime(self.next_lifetime(None)));

        visit_mut::visit_type_impl_trait_mut(self, impl_trait);

        self.impl_trait_ctr -= 1;
    }

    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        generic_args: &mut ParenthesizedGenericArguments,
    ) {
        if self.is_impl_trait() {
            self.impl_fn_ctr += 1;
        }

        visit_mut::visit_parenthesized_generic_arguments_mut(self, generic_args);

        if self.is_impl_trait() {
            self.impl_fn_ctr -= 1;
        }
    }
}
