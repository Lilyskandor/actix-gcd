#[allow(dead_code, unused_imports)]
use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    // '||' is a closure : A value called as if it was a function. From a pool of requests, retrieves the App value that tells how to route and handle requests
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(get_index))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("Error binding server to address")
        .run().expect("Error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post"/>
                    <input type="text" name="First number"/>
                    <input type="text" name="Second number"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}
