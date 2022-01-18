use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use postgres::{Client,NoTls};



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn get_products() -> impl Responder {
    let mut client = match Client::connect(
        "host=localhost user=postgres password=password", NoTls
    ) {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError()
            .body(format!(
                "Cound not connect to DB! {:?}", e)),
    };

    let rows = match client.query(
        "SELECT name, price, image FROM products;",
        &[],
    ){
        Ok(rows) => rows,
        Err(e) => return HttpResponse::InternalServerError()
            .body("Failed DB query"),
    };

    for row in rows {
           println!("{}",row.get::<&str, Option<String>>("name").unwrap());
           println!("{}",row.get::<&str, Option<String>>("image").unwrap());
           println!("{}",row.get::<&str, Option<String>>("price").unwrap());
       println!(
           "Product row: {}, price: {}, image: {}",
           row.get::<&str, Option<String>>("name").unwrap(),
           row.get::<&str, Option<String>>("price").unwrap(),
           row.get::<&str, Option<String>>("image").unwrap(),
       );
    }
    HttpResponse::Ok().body("products here")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET","POST"])
            .max_age(3600);
        App::new()
            .service(hello)
            .service(echo)
            .wrap(cors)
            .route("/hey", web::get().to(manual_hello))
            .route("/products", web::get().to(get_products))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
