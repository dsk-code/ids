pub mod delete_teacher;
pub mod get_teacher;
pub mod get_teachers_list;
pub mod patch_teacher;
pub mod post_teachers;
pub mod put_teacher;

use crate::model::auth_user::{AuthUser, AuthUserExt};
use delete_teacher::delete_teacher;
use get_teacher::get_teacher;
use get_teachers_list::get_teachers_list;
use post_teachers::post_teachers;

use axum::{
    routing::{get, post, put},
    Router,
};
use put_teacher::put_teacher;

pub fn router<A: AuthUserExt>() -> Router {
    Router::new()
        .route(
            "/",
            post(post_teachers::<AuthUser>).get(get_teachers_list::<AuthUser>),
        )
        .route(
            "/{teacher_id}",
            get(get_teacher::<AuthUser>).delete(delete_teacher::<AuthUser>),
        )
        .route("/{teacher_id}", put(put_teacher::<AuthUser>))
}
