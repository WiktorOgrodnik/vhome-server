use tide::{Route, Middleware, Next, Redirect};
use crate::session_utils;
use crate::roles::{Roles, AuthorizeLevel};

pub trait AuthorizeRouteExt {
    fn authorized(&mut self, roles: Vec<Roles>) -> &mut Self;
    fn authorized_group(&mut self, roles: Vec<Roles>) -> &mut Self;
}

impl<'a> AuthorizeRouteExt for Route<'a, crate::State> {
    fn authorized(&mut self, roles: Vec<Roles>) -> &mut Self {
        self.with(MustAuthenticateMiddleWare {
            must_be_in_group: false,
            roles,
        })
    }

    fn authorized_group(&mut self, roles: Vec<Roles>) -> &mut Self {
        self.with(MustAuthenticateMiddleWare {
            must_be_in_group: true,
            roles,
        })
    }
}

struct MustAuthenticateMiddleWare {
    must_be_in_group: bool,
    roles: Vec<Roles>,
}

pub async fn authorize(request: &crate::Request, level: AuthorizeLevel, group_id: Option<i32>) -> bool {
    session_utils::get_user_group_session_ind(request)
        .await
        .map(|group_ind| group_ind.roles
            .iter()
            .any(|role| role.has_authority(level)) &&
            match group_id {
                Some(group_id) => group_id == group_ind.group.id,
                None => true,
            })
        .unwrap_or(false)
}

impl MustAuthenticateMiddleWare {
    fn required_authorization_level(&self) -> AuthorizeLevel {
        if self.roles.contains(&Roles::Admin) {
            return AuthorizeLevel::Manage;
        }

        if self.roles.contains(&Roles::Member) {
            return AuthorizeLevel::Edit;
        }

        if self.roles.contains(&Roles::Guest) {
            return AuthorizeLevel::Show;
        }

        AuthorizeLevel::None
    }
}

#[tide::utils::async_trait]
impl Middleware<crate::State> for MustAuthenticateMiddleWare {
    async fn handle(&self, request: crate::Request, next: Next<'_, crate::State>) -> tide::Result {
        
        let pred = match self.must_be_in_group {
            true  => authorize(&request, self.required_authorization_level(), None).await,
            false => session_utils::get_user(&request).await.is_ok(),
        };

        Ok(match pred {
            true => next.run(request).await,
            false => Redirect::new("/").into(),
        })
    }
}
