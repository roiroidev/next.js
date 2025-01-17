use swc_ecmascript::{
    ast::*,
    visit::{Visit, VisitWith},
};

pub(crate) fn contains_cjs(m: &Module) -> bool {
    let mut v = CjsFinder::default();
    m.visit_with(&mut v);
    v.found
}

#[derive(Copy, Clone, Default)]
struct CjsFinder {
    found: bool,
}

/// This visitor implementation supports typescript, because the api of `swc`
/// does not support changing configuration based on content of the file.
impl Visit for CjsFinder {
    fn visit_member_expr(&mut self, e: &MemberExpr) {
        if !e.computed {
            match &e.obj {
                ExprOrSuper::Super(_) => {}
                ExprOrSuper::Expr(obj) => match &**obj {
                    Expr::Ident(obj) => match &*e.prop {
                        Expr::Ident(prop) => {
                            if &*obj.sym == "module" && &*prop.sym == "exports" {
                                self.found = true;
                                return;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
            }
        }

        e.obj.visit_with(self);

        if e.computed {
            e.prop.visit_with(self);
        }
    }
}
