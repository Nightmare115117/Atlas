use axum::Router;

pub trait Route {
    type State: Clone + Send + Sync + 'static;

    fn routes() -> Router<Self::State>;
}