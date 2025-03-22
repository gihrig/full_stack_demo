#[cfg(feature = "ssr")]
mod ssr_imports {
    use axum::extract::State;
    pub use axum::{
        body::Body as AxumBody,
        extract::Path,
        http::Request,
        response::{IntoResponse, Response},
    };
    use full_stack_demo::app::shell;
    use leptos::{config::LeptosOptions, context::provide_context};

    // This custom handler lets us provide Axum State via context
    pub async fn custom_handler(
        Path(id): Path<String>,
        State(options): State<LeptosOptions>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_app_to_stream_with_context(
            move || {
                provide_context(id.clone());
            },
            move || shell(options.clone()),
        );
        handler(req).await.into_response()
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use ssr_imports::custom_handler;
    use full_stack_demo::app::{shell, App, ssr::db};
    use axum::{
        routing::get,
        Router,
    };
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // Initialize logger
    simple_logger::init_with_level(log::Level::Error)
        .expect("couldn't initialize logging");

    // Connect to DB
    let mut conn = db().await.expect("couldn't connect to DB");
    if let Err(e) = sqlx::migrate!().run(&mut conn).await {
        eprintln!("{e:?}");
    }

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/special/:id", get(custom_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
