mod file;
mod posts;

use axum::{extract::Path, http::StatusCode, response::Html, routing::get, Router};
use itertools::Itertools;

const TITLE: &str = "Joe Fioti";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/static/*path", get(static_files))
        .route("/post/*post", get(posts::post))
        .route("/draft/*post", get(posts::draft))
        .route("/drafts", get(drafts))
        .route("/", get(index));

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn static_files(Path(path): Path<String>) -> Result<file::File, StatusCode> {
    match file::File::load(format!("{}/static/{path}", env!("CARGO_MANIFEST_DIR")))
        .await {
        Ok(f) => Ok(f.compress().unwrap()),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn index() -> Html<String> {
    make_list_page(
        TITLE,
        &get_post_links(
            &format!("{}/posts/release", env!("CARGO_MANIFEST_DIR")),
            "post",
        )
        .join("<br/><br/>"),
    )
}

async fn drafts() -> Html<String> {
    make_list_page(
        "Drafts",
        &get_post_links(
            &format!("{}/posts/draft", env!("CARGO_MANIFEST_DIR")),
            "draft",
        )
        .join("<br/><br/>"),
    )
}

fn get_post_links(folder: &str, prefix: &str) -> Vec<String> {
    std::fs::read_dir(folder)
        .unwrap()
        .flatten()
        .map(|f| {
            f.file_name()
                .as_os_str()
                .to_str()
                .unwrap()
                .replace(".md", "")
        })
        .sorted()
        .rev()
        .map(|i| {
            format!(
                "<a class=\"glow\" href=\"{prefix}/{i}\">{}</a>",
                i.split('|').skip(1).join("|").trim()
            )
        })
        .collect::<Vec<_>>()
}

fn make_list_page(title: &str, post_list: &str) -> Html<String> {
    Html(format!(
        r#"
<!doctype html>
<html lang="en">
<head>
    <link rel="shortcut icon" href="https://www.sidekickai.co/favicon.ico?v=2"/>
    <meta charset="utf-8" /> 
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />


    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <title>{title}</title>
    <meta name="description" content="{title}"/>
    <meta name="google-site-verification" content="google-site-verification=1CU1YSszDUxoVo65pJe9UZKWFgC12KXRuDGIUEfcEY0"/>
    <meta name="theme-color" content="rgb(255,255,255)"/>
    <meta name="msapplication-navbutton-color" content="rgb(255,255,255)"/>
    <meta name="apple-mobile-web-app-capable" content="yes"/>
    <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent"/>

    <link rel="stylesheet" type="text/css" href="/static/global.css">
</head>

<body>
    <style>
        a {{
            font-size: 20px;
            text-decoration: none;
            transition: all;
            transition-duration: 200ms;
        }}
    </style>
    <div>
        <h1>{title}</h1>
        {post_list}
    </div>
</body>
</html>
    "#
    ))
}
