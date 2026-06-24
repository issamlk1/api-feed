use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Record {
    pub id: Uuid,
    pub payload: String,      
    pub created_at: String,    
}

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub date: Option<NaiveDate>,
}