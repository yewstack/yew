use syn::visit_mut::VisitMut;

#[derive(Debug, Default)]
pub struct BodyRewriter {
    branch_ctr: u64,
}

impl BodyRewriter {
    fn is_branched(&self) -> bool {
        self.branch_ctr > 0
    }

    fn with_branch<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce() -> O,
    {
        self.branch_ctr += 1;
        let result = f();
        self.branch_ctr -= 1;
        result
    }
}

impl VisitMut for BodyRewriter {}
