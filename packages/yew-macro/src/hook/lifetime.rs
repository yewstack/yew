use std::sync::{Arc, Mutex};

use proc_macro2::Span;
use syn::visit_mut::{self, VisitMut};
use syn::{
    GenericArgument, Lifetime, ParenthesizedGenericArguments, Receiver, TypeBareFn, TypeImplTrait,
    TypeParamBound, TypeReference, TypeTraitObject,
};

// borrowed from the awesome async-trait crate.
pub struct CollectLifetimes {
    pub elided: Vec<Lifetime>,
    pub explicit: Vec<Lifetime>,
    pub name: &'static str,
    pub default_span: Span,

    pub type_trait_obj_lock: Arc<Mutex<()>>,
    pub impl_trait_lock: Arc<Mutex<()>>,
    pub impl_fn_lock: Arc<Mutex<()>>,
}

impl CollectLifetimes {
    pub fn new(name: &'static str, default_span: Span) -> Self {
        CollectLifetimes {
            elided: Vec::new(),
            explicit: Vec::new(),
            name,
            default_span,

            impl_trait_lock: Arc::default(),
            type_trait_obj_lock: Arc::default(),
            impl_fn_lock: Arc::default(),
        }
    }

    fn is_impl_trait(&self) -> bool {
        self.impl_trait_lock.try_lock().is_err()
    }

    fn is_type_trait_obj(&self) -> bool {
        self.type_trait_obj_lock.try_lock().is_err()
    }

    fn is_impl_fn(&self) -> bool {
        self.impl_fn_lock.try_lock().is_err()
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
        // We don't rewrite references in the impl FnOnce(&arg) or fn(&arg)
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
        let impl_trait_lock = self.impl_trait_lock.clone();
        let _locked = impl_trait_lock.try_lock();

        impl_trait
            .bounds
            .insert(0, TypeParamBound::Lifetime(self.next_lifetime(None)));

        visit_mut::visit_type_impl_trait_mut(self, impl_trait);
    }

    fn visit_type_trait_object_mut(&mut self, type_trait_obj: &mut TypeTraitObject) {
        let type_trait_obj_lock = self.type_trait_obj_lock.clone();
        let _locked = type_trait_obj_lock.try_lock();

        visit_mut::visit_type_trait_object_mut(self, type_trait_obj);
    }

    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        generic_args: &mut ParenthesizedGenericArguments,
    ) {
        let impl_fn_lock = self.impl_fn_lock.clone();
        let _maybe_locked =
            (self.is_impl_trait() || self.is_type_trait_obj()).then(|| impl_fn_lock.try_lock());

        visit_mut::visit_parenthesized_generic_arguments_mut(self, generic_args);
    }

    fn visit_type_bare_fn_mut(&mut self, i: &mut TypeBareFn) {
        let impl_fn_lock = self.impl_fn_lock.clone();
        let _locked = impl_fn_lock.try_lock();

        visit_mut::visit_type_bare_fn_mut(self, i);
    }
}
