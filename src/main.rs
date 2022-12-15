#[allow(dead_code, unused_imports)]
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// This deserialize (from serde's crate) will take any kind of form data (like JSON, YAML, TOML, etc..) and parse it into the built structures
#[derive(Deserialize)]
struct GcdParameters {
    first_term: u64,
    second_term: u64,
}

fn main() {
    // '||' is a closure : A value called as if it was a function. From a pool of requests, retrieves the App value that tells how to route and handle requests
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(get_index))// Give all GET requests from the path / to the function get_index
            .route("/gcd", web::post().to(post_gcd)) // Give all POST requests from the path /gcd to the function post_gcd
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("Error binding server to address")
        .run().expect("Error running server");
}

// Generate the form and send it back
fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post"/>
                    <input type="text" name="first_term"/>
                    <input type="text" name="second_term"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
        ) // This is a raw string: r#"<AnyCharacters>"# will interpret every character as part of the string, without the need to escape it. Add more # to avoid misinterpreting "#
}

// Get the parsed form, and computes the gcd before sending the answer back
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    // Reject 0's by sending a badrequest
    if form.first_term == 0 || form.second_term == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Please type only non-zero integers");
    }

    // Prepare the response by formating the output string
    let response =
        format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.first_term, form.second_term, greatest_common_divisor(form.first_term, form.second_term));

    // Generate the HttpResponse
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

// Computes the Greatest Common Divisor
fn greatest_common_divisor(mut first_term: u64, mut second_term: u64) -> u64 {
    assert!(first_term != 0 && second_term != 0); // Panics if 0's given. Relevant chcek or not?
    while second_term != 0 {
        if second_term < first_term {
            let tmp = second_term;
            second_term = first_term;
            first_term = tmp;
        }
        second_term = second_term % first_term;
    }
    first_term
}

// Unit test
#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(14, 15), 1);

    assert_eq!(greatest_common_divisor(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11)
}
