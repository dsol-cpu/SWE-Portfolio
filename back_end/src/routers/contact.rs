use actix_web::{ web, HttpResponse, Result };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContactForm {
    name: String,
    email: String,
    message: String,
}

// Contact Form Handler
pub async fn submit_contact(form: web::Json<ContactForm>) -> Result<HttpResponse> {
    // Here you would typically:

    // 1. Validate the email
    // validateEmail(form.email);
    // 2. Save to database
    // post("supabase api->");

    // 3. Send confirmation email
    // 4. Maybe notify yourself of new contact

    Ok(HttpResponse::Ok().json("Message received"))
}
