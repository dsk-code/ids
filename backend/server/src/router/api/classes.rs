pub mod delete_class;
pub mod get_class;
pub mod get_class_list;
pub mod post_classes;
pub mod put_class;

use crate::model::auth_user::{AuthUser, AuthUserExt};

use delete_class::delete_class;
use get_class::get_class;
use get_class_list::get_classes_list;
use post_classes::post_class;
use put_class::update_class;

use axum::{
    routing::{post, put},
    Router,
};

pub fn router<A: AuthUserExt>() -> Router {
    Router::new()
        .route(
            "/",
            post(post_class::<AuthUser>).get(get_classes_list::<AuthUser>),
        )
        .route(
            "/{class_id}",
            put(update_class::<AuthUser>)
                .get(get_class::<AuthUser>)
                .delete(delete_class::<AuthUser>),
        )
}
