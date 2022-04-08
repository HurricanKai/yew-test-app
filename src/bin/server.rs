use actix_files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use yew::ServerRenderer;
use yew_app::{ServerApp as MainComponent, ServerAppProps};

lazy_static! {
    static ref INDEX_HTML: String =
        std::fs::read_to_string("./dist/index.html").expect("index.html is required");
    static ref INDEX_HTML_NO_HYDRATE: String =
        std::fs::read_to_string("./dist/index_no_hydrate.html").expect("index_no_hydrate.html is required");
}

async fn index(req: HttpRequest) -> impl Responder {
    let server_app_props = ServerAppProps {
        history: req.uri().to_string(),
    };

    let renderer = ServerRenderer::<MainComponent>::with_props(server_app_props).hydratable(true);

    let mut content = String::new();
    renderer.render_to_string(&mut content).await;
    HttpResponse::Ok().body(INDEX_HTML.replace("<body>", &format!("<body>{}", content)))
}

async fn index_no_hydrate(req: HttpRequest) -> impl Responder {
    let server_app_props = ServerAppProps {
        history: req.uri().to_string(),
    };

    let renderer = ServerRenderer::<MainComponent>::with_props(server_app_props).hydratable(false);

    let mut content = String::new();
    renderer.render_to_string(&mut content).await;
    HttpResponse::Ok().body(INDEX_HTML_NO_HYDRATE.replace("<body>", &format!("<body>{}", content)))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index_no_hydrate))
            .route("/counter", web::get().to(index))
            .route("/loading", web::get().to(index))
            .route("/nested", web::get().to(index_no_hydrate))
            .route("/nested/{tail:.*}", web::get().to(index))
            .route("/404", web::get().to(index))
            .default_service(actix_files::Files::new("/*", "./dist"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
