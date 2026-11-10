use crate::user::UserService;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UserState {
    pub service: Arc<UserService>,
}