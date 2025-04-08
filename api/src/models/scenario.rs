use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Scenario {
    pub id: i32,
    pub title: String,
    pub type_name: String,
    pub temperature_target: Option<String>,
    pub description: Option<String>,
    pub publisher_id: Option<i32>,
    pub published_date: Option<NaiveDate>,
    pub target_year: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stakeholder {
    pub id: i32,
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sector {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioDetail {
    pub id: i32,
    pub title: String,
    pub type_name: String,
    pub temperature_target: Option<String>,
    pub description: Option<String>,
    pub published_date: Option<NaiveDate>,
    pub target_year: Option<i32>,
    pub publisher: Option<Publisher>,
    pub regions: Vec<Region>,
    pub stakeholders: Vec<Stakeholder>,
    pub sectors: Vec<Sector>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ScenarioListItem {
    pub id: i32,
    pub title: String,
    pub type_name: String,
    pub temperature_target: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<NaiveDate>,
    pub target_year: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ScenarioFilters {
    pub publisher_id: Option<i32>,
    pub region_id: Option<i32>,
    pub stakeholder_id: Option<i32>,
    pub sector_id: Option<i32>,
    pub type_name: Option<String>,
    pub temperature_target: Option<String>,
    pub year_from: Option<i32>,
    pub year_to: Option<i32>,
}