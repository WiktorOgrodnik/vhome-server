use axum::{
    middleware,
    response::Redirect,
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{
    middleware::{
        requires_authentication::requires_authentication, requires_group::requires_group,
    },
    routes::{
        device::{
            add_device::add_device,
            delete_device::delete_device,
            edit_device::edit_device,
            get_devices::get_devices,
            get_measurements::get_measurements,
            thermometer::{
                get_thermometer::get_thermometer, update_thermometer::update_thermometer,
            },
        },
        display::{
            get_pairing_code::get_pairing_code, pair_display::pair_display,
            use_pairing_code::use_pairing_code,
        },
        greet::default as greet,
        group::{
            accept_invitation::accept_invitation,
            add_group::add_group,
            generate_group_invitation::generate_group_invitation,
            get_groups::get_groups,
            leave_group::leave_group,
            select_group::{select_group, unselect_group},
        },
        task::{
            add_task::add_task,
            delete_task::delete_task,
            edit_task::edit_task,
            get_task::get_task,
            get_task_assigns::get_task_assigns,
            get_tasks::get_tasks,
            set_task_assign::{set_assign, set_unassing},
            set_task_completed::{set_completed, set_uncompleted},
        },
        taskset::{
            add_taskset::add_taskset, delete_taskset::delete_taskset,
            get_group_tasksets::get_group_tasksets, get_taskset::get_taskset,
        },
        user::{
            add_user_picture::add_user_picture, create_user::create_user,
            get_group_users::get_group_users, get_user_picture::get_user_picture, login::login,
            logout::logout,
        },
    },
    state::AppState,
};

pub fn init_router(appstate: AppState) -> Router {
    Router::new()
        .route("/thermometer/:device_id", get(get_thermometer))
        .route("/devices", get(get_devices))
        .route("/devices", post(add_device))
        .route("/device/:device_id", patch(edit_device))
        .route("/device/:device_id", delete(delete_device))
        .route(
            "/measurements/:device_id/:time_range",
            get(get_measurements),
        )
        .route("/taskset/:taskset_id", get(get_taskset))
        .route("/taskset/:taskset_id", delete(delete_taskset))
        .route("/tasksets", get(get_group_tasksets))
        .route("/tasksets", post(add_taskset))
        .route("/task/:task_id", get(get_task))
        .route("/task/:task_id", patch(edit_task))
        .route("/task/:task_id", delete(delete_task))
        .route("/task/:task_id/assign", get(get_task_assigns))
        .route("/task/:task_id/assign/:user_id", put(set_assign))
        .route("/task/:task_id/unassign/:user_id", put(set_unassing))
        .route("/task/:task_id/completed", put(set_completed))
        .route("/task/:task_id/uncompleted", put(set_uncompleted))
        .route("/tasks", post(add_task))
        .route("/tasks/:taskset_id", get(get_tasks))
        .route("/users", get(get_group_users))
        .route(
            "/group/generate_invitation",
            post(generate_group_invitation),
        )
        .route("/group/leave", post(leave_group))
        .route("/display", post(pair_display))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            requires_group,
        ))
        .route("/user/picture", post(add_user_picture))
        .route("/groups", post(add_group))
        .route("/group/accept", post(accept_invitation))
        .route("/group/select/:group_id", get(select_group))
        .route("/group/unselect", get(unselect_group))
        .route("/groups", get(get_groups))
        .route("/logout", get(logout))
        .route_layer(middleware::from_fn_with_state(
            appstate.clone(),
            requires_authentication,
        ))
        .route("/display/pairing_code", get(get_pairing_code))
        .route("/display/pairing_code", post(use_pairing_code))
        .route("/user/:user_id/picture", get(get_user_picture))
        .route("/thermometer", patch(update_thermometer))
        .route("/login", post(login))
        .route("/register", post(create_user))
        .route("/", get(|| async { Redirect::permanent("/home") }))
        .route("/home", get(greet))
        .with_state(appstate)
}
