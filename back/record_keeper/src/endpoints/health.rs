use crate::models::health_check_response::HealthCheckResponse;
use rocket::serde::json::Json;

#[get("/health")]
pub fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "OK".to_string(),
    })
}
