use crate::common::{ServiceG, UserState};
use crate::user::{UserDTO, UserDTOCreate, UserServiceError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::vec::Vec;
use uuid::Uuid;

pub async fn get_all(
    State(state): State<UserState>,
) -> Result<Json<Vec<UserDTO>>, UserServiceError> {
    let users = state.service.get_all().await?;

    Ok(Json(users))
}

pub async fn get_by_id(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserDTO>, UserServiceError> {
    let user = state.service.get_by_id(id).await?;

    Ok(Json(user))
}

pub async fn create(
    State(state): State<UserState>,
    Json(payload): Json<UserDTOCreate>,
) -> Result<StatusCode, UserServiceError> {
    state.service.create(payload).await?;

    Ok(StatusCode::OK)
}

pub async fn update(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserDTOCreate>,
) -> Result<StatusCode, UserServiceError> {
    state.service.update(id, payload).await?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    State(state): State<UserState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, UserServiceError> {
    state.service.delete(id).await?;

    Ok(StatusCode::OK)
}
