use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
struct Input {
    input: String,
}

#[derive(Serialize)]
struct Result {
    result: String,
}

#[derive(Serialize)]
struct ResultReason {
    result: String,
    reason: String,
}

#[derive(PartialEq)]
enum Joy {
    NeedJ,
    NeedO,
    NeedY,
    Done,
    Bad,
}

#[post("/15/nice")]
async fn day15_nice(info: web::Json<Input>) -> HttpResponse {
    info!("> nice: {}", info.input);
    let count = info
        .input
        .chars()
        .filter(|letter| "aeiouy".contains(*letter))
        .count();
    let mut double = false;
    for i in 0..(info.input.len() - 1) {
        let j = i as usize;
        if info.input.chars().nth(j).unwrap().is_alphabetic()
            && info.input.chars().nth(j).unwrap() == info.input.chars().nth(j + 1).unwrap()
        {
            double = true;
            break;
        }
    }
    let forbidden = vec!["ab", "cd", "pq", "xy"]
        .iter()
        .any(|f| info.input.contains(f));
    if count >= 3 && double && !forbidden {
        return HttpResponse::Ok().json(Result {
            result: "nice".to_string(),
        });
    }
    HttpResponse::BadRequest().json(Result {
        result: "naughty".to_string(),
    })
}

#[post("/15/game")]
async fn day15_game(info: web::Json<Input>) -> HttpResponse {
    info!("> game: {}", info.input);

    // Rule 1: must be at least 8 characters long
    if info.input.len() != 8 {
        return HttpResponse::BadRequest().json(ResultReason {
            result: "naughty".to_string(),
            reason: "8 chars".to_string(),
        });
    }

    // Rule 2: must contain uppercase letters, lowercase letters, and digits
    if !info.input.chars().any(|f| f.is_lowercase())
        || !info.input.chars().any(|f| f.is_uppercase())
        || !info.input.chars().any(|f| f.is_digit(10))
    {
        return HttpResponse::BadRequest().json(ResultReason {
            result: "naughty".to_string(),
            reason: "more types of chars".to_string(),
        });
    }

    // Rule 3: must contain at least 5 digits
    if info.input.chars().filter(|f| f.is_digit(10)).count() < 5 {
        return HttpResponse::BadRequest().json(ResultReason {
            result: "naughty".to_string(),
            reason: "55555".to_string(),
        });
    }

    // Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
    let mut str = String::new();
    let mut sum = 0;
    info.input.chars().for_each(|f| {
        if f.is_digit(10) {
            str.push(f);
        } else {
            sum += str.parse::<i32>().unwrap();
            str.clear();
        }
    });
    if sum != 2023 {
        return HttpResponse::BadRequest().json(ResultReason {
            result: "naughty".to_string(),
            reason: "math is hard".to_string(),
        });
    }

    // Rule 5: must contain the letters j, o, and y in that order and in no other order
    let mut state = Joy::NeedJ;
    info.input.chars().for_each(|f| {
        if !"joy".contains(f) || state == Joy::Bad {
            return;
        }

        match state {
            Joy::NeedJ => {
                if f == 'j' {
                    state = Joy::NeedO;
                } else {
                    state = Joy::Bad;
                }
            }
            Joy::NeedO => {
                if f == 'o' {
                    state = Joy::NeedY;
                } else {
                    state = Joy::Bad;
                }
            }
            Joy::NeedY => {
                if f == 'y' {
                    state = Joy::Done;
                } else {
                    state = Joy::Bad;
                }
            }
            _ => {}
        }
    });
    if state != Joy::Done {
        return HttpResponse::NotAcceptable().json(ResultReason {
            result: "naughty".to_string(),
            reason: "not joyful enough".to_string(),
        });
    }

    HttpResponse::BadRequest().json(Result {
        result: "naughty".to_string(),
    })
}
