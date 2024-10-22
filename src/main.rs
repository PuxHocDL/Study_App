pub mod api;
pub mod model;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    use api::{loader::model_loader, ws};
    use candlemist::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(|| view! { <App/> });

    #[get("/style.css")]
    async fn css() -> impl Responder {
        actix_files::NamedFile::open_async("./style/output.css").await
    }

    let (model, tokenizer, device) = model_loader().unwrap();

    let mdl = web::Data::new(model);
    let tkn = web::Data::new(tokenizer);
    let dvc = web::Data::new(device);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(mdl.clone())
            .app_data(tkn.clone())
            .app_data(dvc.clone())
            .service(css)
            .route("/ws", web::get().to(ws))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/> },
            )
            .service(Files::new("/", site_root))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use candlemist::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
