use actix_web::{ HttpResponse, Result };
use tokio::{ fs::File, io::AsyncReadExt };

// Resume Download
pub async fn download_resume() -> Result<HttpResponse> {
    let mut file = File::open("../static/resume.pdf").await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    Ok(
        HttpResponse::Ok()
            .content_type("application/pdf")
            .append_header(("Content-Disposition", "attachment; filename=\"resume.pdf\""))
            .body(contents)
    )
}
