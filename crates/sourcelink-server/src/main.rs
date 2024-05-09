#![warn(missing_docs)]

//! Link shortening service for Sourcelink.
//!
//! Provides an API for the Sourcelink CLI to manage links, including
//! projects.

mod error;
mod models;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Redirect,
    routing, Json, Router,
};
use error::SourcelinkError;
use nanoid::nanoid;
use serde::Deserialize;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::sync::Arc;

use crate::error::Result;
use crate::models::*;

/// Server configuration
#[derive(Deserialize, Clone, Debug)]
struct Config {
    /// URL of the SQLite Database
    database_url: String,
    /// Server hostname
    host: String,
    /// API key for authentication
    api_key: String,
}

/// Context passed to route handlers
#[derive(Debug)]
struct Context {
    /// Service configuration
    config: Config,
    /// Database connection pool
    pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    structured_logger::Builder::with_level("info")
        .with_target_writer(
            "*",
            structured_logger::async_json::new_writer(tokio::io::stdout()),
        )
        .init();
    dotenvy::dotenv().unwrap();

    let config = envy::from_env::<Config>().unwrap();
    let state = Arc::new(Context {
        config: config.clone(),
        pool: SqlitePool::connect(&config.database_url).await.unwrap(),
    });
    let app = Router::new()
        .route(
            "/api/projects",
            routing::get(get_projects).post(create_project),
        )
        .route(
            "/api/project/:id",
            routing::get(get_project).delete(delete_project),
        )
        .route("/api/links", routing::post(create_link))
        .route("/api/link/:id", routing::get(get_link).delete(delete_link))
        .route("/:id", routing::get(redirect_link))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&config.host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Authenticate against the API Key.
fn authenticate(headers: &HeaderMap, ctx: &Context) -> Result<()> {
    match headers.get("X-API-Key") {
        Some(value) => {
            if value.to_str().unwrap() == ctx.config.api_key {
                Ok(())
            } else {
                Err(SourcelinkError::Unauthorized)
            }
        }
        None => Err(SourcelinkError::Unauthorized),
    }
}

/// Fetch all projects.
async fn get_projects(
    headers: HeaderMap,
    State(ctx): State<Arc<Context>>,
) -> Result<(StatusCode, Json<GetProjects>)> {
    log::info!(target: "api",
        method = "GET",
        path = "/api/projects";
        "",
    );

    authenticate(&headers, &ctx)?;

    let projects = sqlx::query_as!(ProjectRow, r#"SELECT id, name FROM projects"#)
        .fetch_all(&ctx.pool)
        .await?;

    Ok((StatusCode::OK, Json(GetProjects { projects })))
}

/// Create a new project.
async fn create_project(
    headers: HeaderMap,
    ctx: State<Arc<Context>>,
    Json(params): Json<CreateProject>,
) -> Result<(StatusCode, Json<ProjectRow>)> {
    log::info!(target: "api",
        method = "POST",
        path = "/api/projects",
        params:serde = params;
        "",
    );

    authenticate(&headers, &ctx)?;

    let id = nanoid!(6);
    let project = sqlx::query_as!(
        ProjectRow,
        r#"
        INSERT INTO projects(id, name) VALUES(?, ?)
        RETURNING id, name
    "#,
        id,
        params.name
    )
    .fetch_one(&ctx.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(project)))
}

/// Get a project by id.
async fn get_project(
    Path(id): Path<String>,
    headers: HeaderMap,
    ctx: State<Arc<Context>>,
) -> Result<Json<Project>> {
    log::info!(target: "api",
        method = "GET",
        path = format!("/api/project/{id}");
        "",
    );

    authenticate(&headers, &ctx)?;

    let project = sqlx::query_as!(
        ProjectRow,
        r#"SELECT id, name FROM projects WHERE id = ?"#,
        id
    )
    .fetch_one(&ctx.pool)
    .await?;
    let links = sqlx::query_as!(
        ProjectLink,
        r#"SELECT id, url FROM links WHERE project_id = ?"#,
        id
    )
    .fetch_all(&ctx.pool)
    .await?;

    Ok(Json(Project {
        id: project.id,
        name: project.name,
        links,
    }))
}

/// Delete a project.
async fn delete_project(
    Path(id): Path<String>,
    headers: HeaderMap,
    ctx: State<Arc<Context>>,
) -> Result<StatusCode> {
    log::info!(target: "api",
        method = "DELETE",
        path = format!("/api/project/{id}");
        ""
    );

    authenticate(&headers, &ctx)?;

    sqlx::query!(r#"DELETE FROM projects WHERE id = ?"#, id)
        .execute(&ctx.pool)
        .await?;
    sqlx::query!(r#"DELETE FROM links WHERE project_id = ?"#, id)
        .execute(&ctx.pool)
        .await?;

    Ok(StatusCode::OK)
}

/// Create a new link.
async fn create_link(
    headers: HeaderMap,
    ctx: State<Arc<Context>>,
    Json(params): Json<CreateLink>,
) -> Result<(StatusCode, Json<LinkRow>)> {
    log::info!(target: "api",
        method = "POST",
        path = "/api/links",
        params:serde = params;
        "",
    );

    authenticate(&headers, &ctx)?;

    let id = nanoid!(6);
    let link = sqlx::query_as!(
        LinkRow,
        r#"INSERT INTO links(id, url, project_id) VALUES (?, ?, ?) RETURNING id, url, project_id"#,
        id,
        params.url,
        params.project_id
    )
    .fetch_one(&ctx.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(link)))
}

/// Get a link by id.
async fn get_link(
    Path(id): Path<String>,
    headers: HeaderMap,
    ctx: State<Arc<Context>>,
) -> Result<Json<Link>> {
    log::info!(target: "api",
        method = "GET",
        path = format!("/api/link/{id}");
        ""
    );

    authenticate(&headers, &ctx)?;

    let link = sqlx::query_as!(
        LinkRow,
        r#"SELECT id, url, project_id FROM links WHERE id = ?"#,
        id
    )
    .fetch_one(&ctx.pool)
    .await?;
    let project = sqlx::query_as!(
        ProjectRow,
        r#"SELECT id, name FROM projects WHERE id = ?"#,
        link.project_id
    )
    .fetch_one(&ctx.pool)
    .await?;

    Ok(Json(Link {
        id: link.id,
        url: link.url,
        project,
    }))
}

/// Delete a link.
async fn delete_link(
    Path(id): Path<String>,
    headers: HeaderMap,
    ctx: State<Arc<Context>>,
) -> Result<StatusCode> {
    log::info!(target: "api",
        method = "DELETE",
        path = format!("/api/link/{id}");
        ""
    );

    authenticate(&headers, &ctx)?;

    sqlx::query!(r#"DELETE FROM links WHERE id = ?"#, id)
        .execute(&ctx.pool)
        .await?;

    Ok(StatusCode::OK)
}

/// Redirect a link to its destination.
async fn redirect_link(Path(id): Path<String>, ctx: State<Arc<Context>>) -> Result<Redirect> {
    log::info!(target: "api",
        method = "GET",
        path = format!("/{id}");
        "",
    );

    let link = sqlx::query!(r#"SELECT url FROM links WHERE id = ?"#, id)
        .fetch_one(&ctx.pool)
        .await?;

    log::info!(target: "api",
        source = id,
        destination = link.url;
        "Redirected"
    );

    Ok(Redirect::permanent(&link.url))
}
