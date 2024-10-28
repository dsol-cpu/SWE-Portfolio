use actix_web::{ HttpResponse, Result };
use serde::Serialize;

#[derive(Serialize)]
struct WeatherData {
    temperature: f32,
    condition: String,
    location: String,
}

// Weather Data
pub async fn get_weather() -> Result<HttpResponse> {
    // Integrate with a weather API
    let weather = WeatherData {
        temperature: 22.5,
        condition: "Sunny".to_string(),
        location: "New York".to_string(),
    };

    Ok(HttpResponse::Ok().json(weather))
}
