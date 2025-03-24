use crate::{error_template::ErrorTemplate, errors::AppError};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[server(CauseInternalServerError, "/api")]
pub async fn cause_internal_server_error() -> Result<(), ServerFnError> {
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Err(ServerFnError::ServerError(
        "Generic Server Error".to_string(),
    ))
}

#[server(NotImplementedError, "/api")]
pub async fn cause_not_implemented_error() -> Result<(), ServerFnError> {
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(250));

    Err(ServerFnError::ServerError(
        "Not Implemented Server Error".to_string(),
    ))
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <title>"Errors"</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <Meta />
                <link rel="stylesheet" id="leptos" href="/pkg/full_stack_demo.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
            </head>
            <body>
                <ErrorApp />
            </body>
        </html>
    }
}

#[component]
pub fn ErrorApp() -> impl IntoView {
  use crate::{error_template::ErrorTemplate, errors::AppError};
  use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
    provide_meta_context();
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Stylesheet id="leptos" href="/pkg/errors_axum.css" />
        <Router>
            <header>
                <h1>"Error Examples:"</h1>
            </header>
            <main>
                <Routes fallback=|| {
                    let mut errors = Errors::default();
                    errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate errors /> }.into_view()
                }>
                    <Route path=StaticSegment("") view=ExampleErrors />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn ExampleErrors() -> impl IntoView {
    let generate_internal_error =
        ServerAction::<CauseInternalServerError>::new();

    view! {
        <p>
            "These links will load 404 pages. Verify with browser development tools: " <br />
            <a href="/non-existent">"This links to a nonexistent page"</a><br />
            <a href="/non-existent" target="_blank">
                "Same link, but opens a new tab"
            </a>
        </p>
        <hr style="border: 2px solid #ccc;" />
        <p>"Generates an InternalServerError. Works with JS/WASM blocked:"</p>
        <ActionForm action=generate_internal_error>
            <input
                name="error1"
                type="submit"
                value="Generate a 500 InternalServerError on Browser Console"
            />
        </ActionForm>
        <hr style="border: 2px solid #ccc;" />
        <p>"The following blocks generate 501 and 500 errors"</p>
        <hr style="border: 2px dashed #ccc;" />
        <div>
            // note that the error boundaries could be placed above in the Router or lower down
            // in a particular route. The generated errors on the entire page contribute to the
            // final status code sent by the server when producing ssr pages.
            <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors /> }>
                <ReturnsNotImplementedError />
            </ErrorBoundary>
        </div>
        <hr style="border: 2px dashed #ccc;" />
        <div>
            // note that the error boundaries could be placed above in the Router or lower down
            // in a particular route. The generated errors on the entire page contribute to the
            // final status code sent by the server when producing ssr pages.
            <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors /> }>
                <ReturnsServerError />
            </ErrorBoundary>
        </div>
    }
}

#[component]
pub fn ReturnsServerError() -> impl IntoView {
    Err::<String, AppError>(AppError::InternalServerError)
}

#[component]
pub fn ReturnsNotImplementedError() -> impl IntoView {
    Err::<String, AppError>(AppError::NotImplementedError)
}
