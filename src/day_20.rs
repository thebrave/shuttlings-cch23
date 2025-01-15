use actix_web::web::Buf;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use tar::Archive;
use tracing::{error, info};

#[post("/20/archive_files")]
async fn day20_number_files(data: web::Bytes, req: HttpRequest) -> Result<HttpResponse, Error> {
    info!("> {}", req.path());
    let mut a = Archive::new(data.reader());
    match a.entries() {
        Ok(entries) => {
            let mut siz = 0u64;
            for entry in entries {
                match entry {
                    Ok(entry) => siz += 1,
                    Err(err) => {
                        error!("! failed to read archive entry: {:?}", err)
                    }
                };
            }
            Ok(HttpResponse::Ok().body(siz.to_string()))
        }
        Err(err) => {
            error!("! failed to open archive: {:?}", err);
            Ok(HttpResponse::BadRequest().finish())
        }
    }
}

#[post("/20/archive_files_size")]
async fn day20_size_files(data: web::Bytes, req: HttpRequest) -> Result<HttpResponse, Error> {
    info!("> {}", req.path());
    let mut a = Archive::new(data.reader());
    let mut siz = 0u64;
    for entry in a.entries()? {
        match entry {
            Ok(entry) => siz += entry.size(),
            Err(err) => {
                error!("! failed to read archive entry: {:?}", err)
            }
        };
    }
    Ok(HttpResponse::Ok().body(siz.to_string()))
}
