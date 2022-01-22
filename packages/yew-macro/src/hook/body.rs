use proc_macro2::Span;
use proc_macro_error::emit_error;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote_spanned, visit_mut, Expr, ExprCall, ExprClosure, ExprForLoop, ExprIf, ExprLoop,
    ExprMatch, ExprWhile, Ident,
};

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
        F: FnOnce(&mut BodyRewriter) -> O,
    {
        self.branch_ctr += 1;
        let result = { f(self) };
        self.branch_ctr -= 1;
        result
    }
}

impl VisitMut for BodyRewriter {
    fn visit_expr_call_mut(&mut self, i: &mut ExprCall) {
        let ctx_ident = Ident::new("ctx", Span::mixed_site());

        // Only rewrite hook calls.
        if let Expr::Path(ref m) = &*i.func {
            if let Some(m) = m.path.segments.last().as_ref().map(|m| &m.ident) {
                if m.to_string().starts_with("use_") {
                    if self.is_branched() {
                        emit_error!(m, "hooks cannot be called at this position.");
                    } else {
                        *i = parse_quote_spanned! { i.span() => ::yew::functional::Hook::run(#i, #ctx_ident) };
                    }

                    return;
                }
            }
        }

        visit_mut::visit_expr_call_mut(self, i);
    }

    fn visit_expr_closure_mut(&mut self, i: &mut ExprClosure) {
        self.with_branch(move |m| visit_mut::visit_expr_closure_mut(m, i))
    }

    fn visit_expr_if_mut(&mut self, i: &mut ExprIf) {
        for it in &mut i.attrs {
            visit_mut::visit_attribute_mut(self, it);
        }

        visit_mut::visit_expr_mut(self, &mut *i.cond);

        self.with_branch(|m| visit_mut::visit_block_mut(m, &mut i.then_branch));

        if let Some(it) = &mut i.else_branch {
            self.with_branch(|m| visit_mut::visit_expr_mut(m, &mut *(it).1));
        }
    }

    fn visit_expr_loop_mut(&mut self, i: &mut ExprLoop) {
        self.with_branch(|m| visit_mut::visit_expr_loop_mut(m, i));
    }

    fn visit_expr_for_loop_mut(&mut self, i: &mut ExprForLoop) {
        for it in &mut i.attrs {
            visit_mut::visit_attribute_mut(self, it);
        }
        if let Some(it) = &mut i.label {
            visit_mut::visit_label_mut(self, it);
        }
        visit_mut::visit_pat_mut(self, &mut i.pat);
        visit_mut::visit_expr_mut(self, &mut *i.expr);

        self.with_branch(|m| visit_mut::visit_block_mut(m, &mut i.body));
    }

    fn visit_expr_match_mut(&mut self, i: &mut ExprMatch) {
        for it in &mut i.attrs {
            visit_mut::visit_attribute_mut(self, it);
        }

        visit_mut::visit_expr_mut(self, &mut *i.expr);

        self.with_branch(|m| {
            for it in &mut i.arms {
                visit_mut::visit_arm_mut(m, it);
            }
        });
    }

    fn visit_expr_while_mut(&mut self, i: &mut ExprWhile) {
        for it in &mut i.attrs {
            visit_mut::visit_attribute_mut(self, it);
        }
        if let Some(it) = &mut i.label {
            visit_mut::visit_label_mut(self, it);
        }

        self.with_branch(|m| visit_mut::visit_expr_mut(m, &mut i.cond));
        self.with_branch(|m| visit_mut::visit_block_mut(m, &mut i.body));
    }
}
