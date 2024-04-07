
use tide::{Route, Middleware, Next, Redirect, Request};
use crate::records::{vgroup::{Participation, UserGroupSessionInd}, vuser};

pub trait AuthorizeRouteExt {
    fn authorized(&mut self, roles: Vec<Participation>) -> &mut Self;
    fn authorized_group(&mut self, roles: Vec<Participation>) -> &mut Self;
}

impl<'a, State> AuthorizeRouteExt for Route<'a, State>
    where State: Clone + Send + Sync + 'static {
    fn authorized(&mut self, roles: Vec<Participation>) -> &mut Self {
        self.with(MustAuthenticateMiddleWare {
            must_be_in_group: false,
            roles,
        })
    }

    fn authorized_group(&mut self, roles: Vec<Participation>) -> &mut Self {
        self.with(MustAuthenticateMiddleWare {
            must_be_in_group: true,
            roles,
        })
    }
}

struct MustAuthenticateMiddleWare {
    must_be_in_group: bool,
    roles: Vec<Participation>,
} 

#[tide::utils::async_trait]
impl<State> Middleware<State> for MustAuthenticateMiddleWare
    where State: Clone + Send + Sync + 'static {
    async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> tide::Result {
        let user: Option<vuser::Data> = request.session().get("user");
        let group: UserGroupSessionInd = request
            .session()
            .get("user_group")
            .unwrap_or_else(|| UserGroupSessionInd::default());

        if user.is_some() && 
            (!self.must_be_in_group ||
            self.roles
                .iter()
                .filter(|elt| group.roles.contains(elt))
                .count() > 0) {
            Ok(next.run(request).await) 
        } else {
            Ok(Redirect::new("/").into())
        }
    }
}
