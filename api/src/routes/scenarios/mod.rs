use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::errors::ApiError;
use crate::models::{ScenarioDetail, ScenarioFilters, ScenarioListItem};

#[get("")]
async fn list_scenarios(
    db: web::Data<PgPool>,
    query: web::Query<ScenarioFilters>,
) -> Result<impl Responder, ApiError> {
    let mut sql = String::from(
        "SELECT s.id, s.title, s.type as type_name, s.temperature_target, 
        s.description, p.name as publisher, s.published_date, s.target_year
        FROM pbtar.scenarios s
        LEFT JOIN pbtar.publishers p ON s.publisher_id = p.id
        WHERE 1=1"
    );

    let mut params: Vec<String> = Vec::new();
    let mut values: Vec<&(dyn sqlx::Encode<sqlx::Postgres> + Sync)> = Vec::new();
    let mut param_count = 1;

    if let Some(publisher_id) = &query.publisher_id {
        sql.push_str(&format!(" AND s.publisher_id = ${}", param_count));
        params.push(publisher_id.to_string());
        values.push(publisher_id);
        param_count += 1;
    }

    if let Some(region_id) = &query.region_id {
        sql.push_str(&format!(" AND s.id IN (SELECT scenario_id FROM pbtar.scenario_regions WHERE region_id = ${})", param_count));
        params.push(region_id.to_string());
        values.push(region_id);
        param_count += 1;
    }

    if let Some(stakeholder_id) = &query.stakeholder_id {
        sql.push_str(&format!(" AND s.id IN (SELECT scenario_id FROM pbtar.scenario_stakeholders WHERE stakeholder_id = ${})", param_count));
        params.push(stakeholder_id.to_string());
        values.push(stakeholder_id);
        param_count += 1;
    }

    if let Some(sector_id) = &query.sector_id {
        sql.push_str(&format!(" AND s.id IN (SELECT scenario_id FROM pbtar.scenario_sectors WHERE sector_id = ${})", param_count));
        params.push(sector_id.to_string());
        values.push(sector_id);
        param_count += 1;
    }

    if let Some(type_name) = &query.type_name {
        sql.push_str(&format!(" AND s.type = ${}", param_count));
        params.push(type_name.to_string());
        values.push(type_name);
        param_count += 1;
    }

    if let Some(temperature_target) = &query.temperature_target {
        sql.push_str(&format!(" AND s.temperature_target = ${}", param_count));
        params.push(temperature_target.to_string());
        values.push(temperature_target);
        param_count += 1;
    }

    if let Some(year_from) = &query.year_from {
        sql.push_str(&format!(" AND s.target_year >= ${}", param_count));
        params.push(year_from.to_string());
        values.push(year_from);
        param_count += 1;
    }

    if let Some(year_to) = &query.year_to {
        sql.push_str(&format!(" AND s.target_year <= ${}", param_count));
        params.push(year_to.to_string());
        values.push(year_to);
        param_count += 1;
    }

    sql.push_str(" ORDER BY s.published_date DESC");

    let scenarios = sqlx::query_as::<_, ScenarioListItem>(&sql)
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    Ok(HttpResponse::Ok().json(scenarios))
}

#[get("/{id}")]
async fn get_scenario(
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, ApiError> {
    let id = path.into_inner();

    let scenario = sqlx::query!(
        r#"
        SELECT 
            s.id, s.title, s.type as "type_name", s.temperature_target, 
            s.description, s.published_date, s.target_year,
            p.id as "publisher_id", p.name as "publisher_name", p.description as "publisher_description"
        FROM pbtar.scenarios s
        LEFT JOIN pbtar.publishers p ON s.publisher_id = p.id
        WHERE s.id = $1
        "#,
        id
    )
    .fetch_optional(db.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?;

    let scenario = match scenario {
        Some(s) => s,
        None => return Err(ApiError::NotFoundError(format!("Scenario with id {} not found", id))),
    };

    let regions = sqlx::query!(
        r#"
        SELECT r.id, r.name, r.parent_id
        FROM pbtar.regions r
        JOIN pbtar.scenario_regions sr ON r.id = sr.region_id
        WHERE sr.scenario_id = $1
        "#,
        id
    )
    .fetch_all(db.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?;

    let stakeholders = sqlx::query!(
        r#"
        SELECT s.id, s.name, s.type as "type_name"
        FROM pbtar.stakeholders s
        JOIN pbtar.scenario_stakeholders ss ON s.id = ss.stakeholder_id
        WHERE ss.scenario_id = $1
        "#,
        id
    )
    .fetch_all(db.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?;

    let sectors = sqlx::query!(
        r#"
        SELECT s.id, s.name
        FROM pbtar.sectors s
        JOIN pbtar.scenario_sectors ss ON s.id = ss.sector_id
        WHERE ss.scenario_id = $1
        "#,
        id
    )
    .fetch_all(db.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?;

    let publisher = if scenario.publisher_id > 0 {
        Some(crate::models::Publisher {
            id: scenario.publisher_id,
            name: scenario.publisher_name,
            description: scenario.publisher_description,
        })
    } else {
        None
    };

    let response = ScenarioDetail {
        id: scenario.id,
        title: scenario.title,
        type_name: scenario.type_name,
        temperature_target: scenario.temperature_target,
        description: scenario.description,
        published_date: scenario.published_date,
        target_year: scenario.target_year,
        publisher,
        regions: regions
            .into_iter()
            .map(|r| crate::models::Region {
                id: r.id,
                name: r.name,
                parent_id: r.parent_id,
            })
            .collect(),
        stakeholders: stakeholders
            .into_iter()
            .map(|s| crate::models::Stakeholder {
                id: s.id,
                name: s.name,
                type_name: s.type_name,
            })
            .collect(),
        sectors: sectors
            .into_iter()
            .map(|s| crate::models::Sector {
                id: s.id,
                name: s.name,
            })
            .collect(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/filters/options")]
async fn get_filter_options(
    db: web::Data<PgPool>,
) -> Result<impl Responder, ApiError> {
    let publishers = sqlx::query!("SELECT id, name FROM pbtar.publishers ORDER BY name")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    let regions = sqlx::query!("SELECT id, name, parent_id FROM pbtar.regions ORDER BY name")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    let stakeholders = sqlx::query!("SELECT id, name, type as type_name FROM pbtar.stakeholders ORDER BY name")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    let sectors = sqlx::query!("SELECT id, name FROM pbtar.sectors ORDER BY name")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    let types = sqlx::query!("SELECT DISTINCT type FROM pbtar.scenarios ORDER BY type")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    let temperature_targets = sqlx::query!("SELECT DISTINCT temperature_target FROM pbtar.scenarios WHERE temperature_target IS NOT NULL ORDER BY temperature_target")
        .fetch_all(db.get_ref())
        .await
        .map_err(|e| ApiError::DbError(e))?;

    #[derive(serde::Serialize)]
    struct FilterOptions {
        publishers: Vec<crate::models::Publisher>,
        regions: Vec<crate::models::Region>,
        stakeholders: Vec<crate::models::Stakeholder>,
        sectors: Vec<crate::models::Sector>,
        types: Vec<String>,
        temperature_targets: Vec<String>,
    }

    let options = FilterOptions {
        publishers: publishers
            .into_iter()
            .map(|p| crate::models::Publisher {
                id: p.id,
                name: p.name,
                description: None,
            })
            .collect(),
        regions: regions
            .into_iter()
            .map(|r| crate::models::Region {
                id: r.id,
                name: r.name,
                parent_id: r.parent_id,
            })
            .collect(),
        stakeholders: stakeholders
            .into_iter()
            .map(|s| crate::models::Stakeholder {
                id: s.id,
                name: s.name,
                type_name: s.type_name,
            })
            .collect(),
        sectors: sectors
            .into_iter()
            .map(|s| crate::models::Sector {
                id: s.id,
                name: s.name,
            })
            .collect(),
        types: types
            .into_iter()
            .filter_map(|t| Some(t.r#type))
            .collect(),
        temperature_targets: temperature_targets
            .into_iter()
            .filter_map(|t| t.temperature_target)
            .collect(),
    };

    Ok(HttpResponse::Ok().json(options))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scenarios")
            .service(list_scenarios)
            .service(get_scenario)
            .service(get_filter_options)
    );
}