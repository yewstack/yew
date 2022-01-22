use proc_macro2::Span;
use syn::visit_mut::{self, VisitMut};
use syn::{GenericArgument, Lifetime, Receiver, TypeReference};

/// Finds an unused lifetime.
pub fn find_available_lifetime(lifetimes: &CollectLifetimes) -> Lifetime {
    for i in 0.. {
        let hook_lifetime = if i == 0 {
            Lifetime::new("'hook", lifetimes.default_span)
        } else {
            Lifetime::new(&format!("'hook{}", i), lifetimes.default_span)
        };

        if !lifetimes.elided.contains(&hook_lifetime)
            && !lifetimes.explicit.contains(&hook_lifetime)
        {
            return hook_lifetime;
        }
    }

    unreachable!()
}

// borrowed from the async-trait crate.
pub struct CollectLifetimes {
    pub elided: Vec<Lifetime>,
    pub explicit: Vec<Lifetime>,
    pub name: &'static str,
    pub default_span: Span,
    // pub fn_ctr: u64,
}

impl CollectLifetimes {
    pub fn new(name: &'static str, default_span: Span) -> Self {
        CollectLifetimes {
            elided: Vec::new(),
            explicit: Vec::new(),
            name,
            default_span,
            // fn_ctr: 0,
        }
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

    // fn is_in_fn(&self) -> bool {
    //     self.fn_ctr > 0
    // }

    // fn with_in_fn<F, O>(&mut self, f: F) -> O
    // where
    //     F: FnOnce(&mut CollectLifetimes) -> O,
    // {
    //     self.fn_ctr += 1;
    //     let result = { f(self) };
    //     self.fn_ctr -= 1;
    //     result
    // }
}

impl VisitMut for CollectLifetimes {
    fn visit_receiver_mut(&mut self, arg: &mut Receiver) {
        if let Some((_, lifetime)) = &mut arg.reference {
            self.visit_opt_lifetime(lifetime);
        }
    }

    fn visit_type_reference_mut(&mut self, ty: &mut TypeReference) {
        self.visit_opt_lifetime(&mut ty.lifetime);
        visit_mut::visit_type_reference_mut(self, ty);
    }

    fn visit_generic_argument_mut(&mut self, gen: &mut GenericArgument) {
        if let GenericArgument::Lifetime(lifetime) = gen {
            self.visit_lifetime(lifetime);
        }
        visit_mut::visit_generic_argument_mut(self, gen);
    }
}
