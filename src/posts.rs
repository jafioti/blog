use std::error::Error;

use axum::{extract::Path, http::StatusCode, response::Html};
use itertools::Itertools;
use markdown::{to_html_with_options, CompileOptions, Options};
use regex::Regex;

fn post_page(path: String, name: &str, back_link: &str) -> Option<String> {
    let md = std::fs::read_to_string(path).ok()?;
    let mut content = to_html_with_options(
        &md,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::default()
            },
            ..Options::default()
        },
    )
    .unwrap();
    content = content
        .replace("<code", "<div class=\"code\"><code")
        .replace("</code>", "</code></div>");
    content = wrap_img_tags(&content).unwrap();
    let page = format!(
        r#"
<!doctype html>
<html lang="en">
<head>
    <link rel="shortcut icon" href="https://www.sidekickai.co/favicon.ico?v=2"/>
    <meta charset="utf-8" /> 
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />


    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <title>{}</title>
    <meta name="description" content="Joe Fioti's Website"/>
    <meta name="google-site-verification" content="google-site-verification=1CU1YSszDUxoVo65pJe9UZKWFgC12KXRuDGIUEfcEY0"/>
    <meta name="theme-color" content="rgb(255,255,255)"/>
    <meta name="msapplication-navbutton-color" content="rgb(255,255,255)"/>
    <meta name="apple-mobile-web-app-capable" content="yes"/>
    <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent"/>

    <link defer href="/static/global.css" rel="stylesheet">
    <link rel="preload" as="video" href="/static/images/icons/loader.webm">
</head>

<body>
    <style>
        .back {{
            font-weight: bold;
            font-size: 15px;
            text-decoration: none;
            transition: all;
            transition-duration: 200ms;
            color: white !important
        }}
        .back:hover {{
            text-shadow: 0 0 1px #fff, 0 0 2px #fff, 0 0 3px #fff;
        }}
        img {{
            max-height: 500px;
            max-width: 100%;
        }}
        a {{
            cursor: pointer;
            color: #3350ff;
            text-decoration: none;
            font-weight: bold;
        }}
    </style>
    <div style="max-width: min(100%, 800px); line-height: 25px;">
    <a class="back" href="{back_link}">Back</a>
    {}
    </div>
</body>
</html>
    "#,
        name.split('|').skip(1).join("|").trim(),
        content
    );

    Some(page)
}

pub async fn post(Path(name): Path<String>) -> Result<Html<String>, StatusCode> {
    match post_page(
        format!("{}/posts/release/{name}.md", env!("CARGO_MANIFEST_DIR")),
        &name,
        "/",
    ) {
        Some(s) => Ok(Html(s)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn draft(Path(name): Path<String>) -> Result<Html<String>, StatusCode> {
    match post_page(
        format!("{}/posts/draft/{name}.md", env!("CARGO_MANIFEST_DIR")),
        &name,
        "/drafts",
    ) {
        Some(s) => Ok(Html(s)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

fn wrap_img_tags(input_html: &str) -> Result<String, Box<dyn Error>> {
    let img_tag_regex = Regex::new(r"(?i)<img[^>]*>")?;
    let output_html = img_tag_regex.replace_all(input_html, |caps: &regex::Captures| {
        format!(r#"<div style="text-align:center;">{}</div>"#, &caps[0])
    });
    Ok(output_html.into_owned())
}
