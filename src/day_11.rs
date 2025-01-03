use actix_multipart::form::{bytes, MultipartForm};
use actix_web::{post, HttpRequest, HttpResponse};
use tracing::info;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    image: bytes::Bytes,
}

#[post("/11/red_pixels")]
async fn day11_redpixels(req: HttpRequest, form: MultipartForm<UploadForm>) -> HttpResponse {
    info!("> red_pixels {}", req.uri().path());
    info!(
        "= form {} {:?}",
        form.image.data.len(),
        form.image.content_type
    );

    let decoder = png::Decoder::new(form.image.data.as_ref());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    let mut count = 0;
    for i in 0..bytes.len()/3 {
        let off= 3*i;
        if bytes[off] as usize > bytes[off+1] as usize + bytes[off+2] as usize {
            count += 1;
        }
    }

    HttpResponse::Ok().body(count.to_string())
}