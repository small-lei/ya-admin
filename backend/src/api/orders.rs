use axum::extract::{Json, Path, Query, State};
use axum::routing::{get, post, put, delete};
use axum::Router;
use num_traits::{cast::ToPrimitive, FromPrimitive};
use sea_orm::{*, prelude::Decimal};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Utc};

use crate::db::entities::order;
use crate::error::{AppError, Result};
use crate::middleware::auth::AuthContext;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub customer_name: String,
    pub phone: String,
    pub prescription: String,
    pub frame_type: String,
    pub lens_type: String,
    pub total_amount: f64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderRequest {
    pub customer_name: Option<String>,
    pub phone: Option<String>,
    pub prescription: Option<String>,
    pub frame_type: Option<String>,
    pub lens_type: Option<String>,
    pub total_amount: Option<f64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListOrdersQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub customer_name: String,
    pub phone: String,
    pub prescription: String,
    pub frame_type: String,
    pub lens_type: String,
    pub total_amount: f64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ListOrdersResponse {
    pub items: Vec<OrderResponse>,
    pub total: u64,
}

// Define an AppState struct to hold both database connection and auth context
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub auth: Arc<AuthContext>,
}

pub fn orders_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_order))
        .route("/", get(list_orders))
        .route("/:id", put(update_order))
        .route("/:id", delete(delete_order))
}

async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>> {
    let order = order::ActiveModel {
        customer_name: Set(req.customer_name),
        phone: Set(req.phone),
        prescription: Set(req.prescription),
        frame_type: Set(req.frame_type),
        lens_type: Set(req.lens_type),
        total_amount: Set(Decimal::from_f64(req.total_amount).unwrap()),
        status: Set(req.status),
        created_by: Set(state.auth.user_id),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let order = order.insert(&state.db).await?;

    Ok(Json(OrderResponse {
        id: order.id,
        customer_name: order.customer_name,
        phone: order.phone,
        prescription: order.prescription,
        frame_type: order.frame_type,
        lens_type: order.lens_type,
        total_amount: order.total_amount.to_f64().unwrap(),
        status: order.status,
        created_at: order.created_at.into(),
        updated_at: order.updated_at.into(),
    }))
}

async fn list_orders(
    State(state): State<AppState>,
    Query(query): Query<ListOrdersQuery>,
) -> Result<Json<ListOrdersResponse>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let paginator = order::Entity::find()
        .filter(order::Column::CreatedBy.eq(state.auth.user_id))
        .order_by_desc(order::Column::CreatedAt)
        .paginate(&state.db, page_size);

    let total = paginator.num_items().await?;
    let orders = paginator.fetch_page(page - 1).await?;

    let items = orders
        .into_iter()
        .map(|order| OrderResponse {
            id: order.id,
            customer_name: order.customer_name,
            phone: order.phone,
            prescription: order.prescription,
            frame_type: order.frame_type,
            lens_type: order.lens_type,
            total_amount: order.total_amount.to_f64().unwrap(),
            status: order.status,
            created_at: order.created_at.into(),
            updated_at: order.updated_at.into(),
        })
        .collect();

    Ok(Json(ListOrdersResponse { items, total }))
}

async fn update_order(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateOrderRequest>,
) -> Result<Json<OrderResponse>> {
    let order = order::Entity::find_by_id(id)
        .filter(order::Column::CreatedBy.eq(state.auth.user_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    let mut order: order::ActiveModel = order.into();

    if let Some(customer_name) = req.customer_name {
        order.customer_name = Set(customer_name);
    }
    if let Some(phone) = req.phone {
        order.phone = Set(phone);
    }
    if let Some(prescription) = req.prescription {
        order.prescription = Set(prescription);
    }
    if let Some(frame_type) = req.frame_type {
        order.frame_type = Set(frame_type);
    }
    if let Some(lens_type) = req.lens_type {
        order.lens_type = Set(lens_type);
    }
    if let Some(total_amount) = req.total_amount {
        order.total_amount = Set(Decimal::from_f64(total_amount).unwrap());
    }
    if let Some(status) = req.status {
        order.status = Set(status);
    }
    order.updated_at = Set(chrono::Utc::now().into());

    let order = order.update(&state.db).await?;

    Ok(Json(OrderResponse {
        id: order.id,
        customer_name: order.customer_name,
        phone: order.phone,
        prescription: order.prescription,
        frame_type: order.frame_type,
        lens_type: order.lens_type,
        total_amount: order.total_amount.to_f64().unwrap(),
        status: order.status,
        created_at: order.created_at.into(),
        updated_at: order.updated_at.into(),
    }))
}

async fn delete_order(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<()> {
    let order = order::Entity::find_by_id(id)
        .filter(order::Column::CreatedBy.eq(state.auth.user_id))
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    let _order = order.delete(&state.db).await?;

    Ok(())
}