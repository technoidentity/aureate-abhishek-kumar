use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct GCDParameters {
    n: u64,
    m: u64,
}

// Serialize: Converts your Rust structure structs or
// enums into readable format such as XML or JSON

// Deserialize does the vice versa 

async fn post_gcd(form: web::Form<GCDParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with 0 is 0 only. ")

    }
    let response = format!(
        "The GCD of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m),
    );
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(||{App::new().route("/", web::get().to(get_index))});
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("errorr running server");
    
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title> GCD Calculator </title>
                <form action = "/gcd" method="post">
                <input type = "text" name = "m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}
