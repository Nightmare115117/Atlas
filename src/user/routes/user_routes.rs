use crate::common::{Route, UserState};
use crate::user::{create, delete as delete_user, get_all, get_by_id, update};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub struct UserRoutes;

impl Route for UserRoutes {
    type State = UserState;

    fn routes() -> Router<Self::State> {
        Router::new()
            .route("/", get(get_all))
            .route("/{id}", get(get_by_id))
            .route("/", post(create))
            .route("/{id}", patch(update))
            .route("/{id}", delete(delete_user))
    }
}
