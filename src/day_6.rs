use actix_web::{post, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Deserialize)]
struct ElfSearch {
    elf: usize,
    #[serde(rename(
        serialize = "shelf with no elf on it",
        deserialize = "shelf with no elf on it"
    ))]
    shelf_wo_elf: usize,
    #[serde(rename(serialize = "elf on a shelf", deserialize = "elf on a shelf"))]
    elf_with_shelf: usize,
}

#[post("/6")]
async fn day6_search(text: String) -> HttpResponse {
    let mut elf_with_shelf = 0;
    let mut shelf_wo_elf = 0;
    for i in 0..text.len() {
        if text[i..].starts_with("elf on a shelf") {
            elf_with_shelf += 1;
        } else if text[i..].starts_with("shelf") && i > 8 && !text[i - 9..i].contains("elf on a ") {
            shelf_wo_elf += 1;
        }
    }

    HttpResponse::Ok().json(ElfSearch {
        elf: text.matches("elf").count(),
        shelf_wo_elf: shelf_wo_elf,
        elf_with_shelf: elf_with_shelf,
    })
}

#[cfg(test)]
mod tests {
    use actix_web::http::header::ContentType;
    use actix_web::http::Method;
    use actix_web::{test, App};

    use crate::day_6::{day6_search, ElfSearch};

    #[actix_web::test]
    async fn task_2_test_2() {
        let app = test::init_service(App::new().service(day6_search)).await;

        // Create request object
        let req = test::TestRequest::with_uri("/6")
            .insert_header(ContentType::plaintext())
            .method(Method::POST)
            .set_payload("In Belfast I heard an elf on a shelf on a shelf on a ")
            .to_request();

        // Call application
        let res: ElfSearch = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            res,
            ElfSearch {
                elf: 4,
                shelf_wo_elf: 0,
                elf_with_shelf: 2
            }
        );
    }

    #[actix_web::test]
    async fn task_2_test_3() {
        let app = test::init_service(App::new().service(day6_search)).await;

        // Create request object
        let req = test::TestRequest::with_uri("/6")
        .   insert_header(ContentType::plaintext())
            .method(Method::POST)
            .set_payload("Somewhere in Belfast under a shelf store but above the shelf realm there's an elf on a shelf on a shelf on a shelf on a elf on a shelf on a shelf on a shelf on a shelf on a elf on a elf on a elf on a shelf on a ")
            .to_request();

        // Call application
        let res: ElfSearch = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            res,
            ElfSearch {
                elf: 16,
                shelf_wo_elf: 2,
                elf_with_shelf: 8
            }
        );
    }
}
