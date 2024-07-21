use axum::{
    middleware,
    response::Redirect,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    middleware::{
        requires_authentication::requires_authentication, requires_group::requires_group,
    },
    routes::{
        greet::default as greet,
        group::{
            get_groups::get_groups,
            select_group::{select_group, unselect_group},
        },
        task::{
            add_task::add_task,
            delete_task::delete_task,
            get_task::one as get_task,
            get_tasks::all as get_tasks,
            set_task_completed::{set_completed, set_uncompleted},
        },
        taskset::{
            add_taskset::add_taskset, delete_taskset::delete_taskset,
            get_all_group_tasksets::all as get_all_group_tasksets,
            get_one_taskset::one as get_one_taskset,
        },
        user::{login::login, logout::logout},
    },
    state::AppState,
};

pub fn init_router(appstate: AppState) -> Router {
    Router::new()
        .route("/taskset/:taskset_id", get(get_one_taskset))
        .route("/taskset/:taskset_id", delete(delete_taskset))
        .route("/tasksets", get(get_all_group_tasksets))
        .route("/tasksets", post(add_taskset))
        .route("/task/:task_id", get(get_task))
        .route("/task/:task_id", delete(delete_task))
        .route("/task/:task_id/completed", put(set_completed))
        .route("/task/:task_id/uncompleted", put(set_uncompleted))
        .route("/tasks", post(add_task))
        .route("/tasks/:taskset_id", get(get_tasks))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            requires_group,
        ))
        .route("/group/select/:group_id", get(select_group))
        .route("/group/unselect", get(unselect_group))
        .route("/groups", get(get_groups))
        .route("/logout", get(logout))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            requires_authentication,
        ))
        .route("/login", post(login))
        .route("/", get(|| async { Redirect::permanent("/home") }))
        .route("/home", get(greet))
        .with_state(appstate)
}
