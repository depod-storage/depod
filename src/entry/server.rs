use std::env;

use actix_web::{App, HttpServer, web};

use crate::{
    api,
    backend::{config::Config, tls::load_tls_config},
    entry::check_bootstrap,
    health,
};

pub async fn server(
    config: Config,
    host: String,
    port: u16,
    https: bool,
    cookie: bool,
) -> std::io::Result<()> {
    let allowed_hosts = config.allowed_hosts().to_vec();
    let mut app_state = config.move_app_state();

    let bootstrap = match check_bootstrap(app_state.pool()).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read app_config: {e}");
            return Ok(());
        }
    };

    if !bootstrap {
        println!("Please use `depod admin` before first run!");
        return Ok(());
    }
    app_state.set_https(https);
    app_state.set_cookie_secure(cookie);

    let server = HttpServer::new(move || {
        let mut cors = actix_cors::Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        for orgin in &allowed_hosts {
            cors = cors.allowed_origin(orgin);
        }

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(health)
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(
                            web::scope("/admin")
                                .service(api::v1::admin::create_new_user)
                                .service(api::v1::admin::users)
                                .service(api::v1::admin::add_perm)
                                .service(api::v1::admin::delete_perm),
                        )
                        .service(
                            web::scope("/auth")
                                .service(api::v1::auth::login)
                                .service(api::v1::auth::me),
                        )
                        .service(
                            web::scope("/project")
                                .service(api::v1::project::create_new_project)
                                .service(api::v1::project::get_projects),
                        )
                        .service(
                            web::scope("/brand")
                                .service(api::v1::brand::create_new_brand)
                                .service(api::v1::brand::get_brands)
                                .service(api::v1::brand::get_models_of_brand)
                                .service(api::v1::brand::get_brand)
                        )
                        .service(
                            web::scope("/model")
                                .service(api::v1::model::create_new_model)
                                .service(api::v1::model::get_models)
                                .service(api::v1::model::get_model)
                        )
                        .service(
                            web::scope("/item")
                                .service(api::v1::item::create_new_item)
                                .service(api::v1::item::get_items),
                        ),
                ),
            )
    });

    if !https {
        server.bind((host.as_str(), port))?.run().await
    } else {
        let cert = env::var("TLS_CERT").expect("TLS_CERT doesnt exist in env");
        let key = env::var("TLS_KEY").expect("TLS_KEY doesnt exist in env");
        let tls = load_tls_config(&cert, &key)?;
        server
            .bind_rustls_0_23((host.as_str(), port), tls)?
            .run()
            .await
    }
}
