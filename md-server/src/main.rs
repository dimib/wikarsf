use std::path::{Path, PathBuf};
use md_html::md::{read_content_from_file, parse};
use md_html::html::generator::{generate_html, StyleType, Pager };

#[macro_use] extern crate rocket;

#[derive(Responder)]
#[response(status = 200, content_type = "text/html")]
struct HtmlResponder(String);


#[get("/<path..>")]
fn page(path: PathBuf) -> HtmlResponder {

    let file_path = Path::new("/Users/dimi/_devel/rust/wikarsf/md-server/content").join(path.as_path());

    match load_page(&file_path) {
        Some(page) => HtmlResponder(page),
        None => HtmlResponder("Not found".to_string()),
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![page])
}

fn load_page(path: &Path) -> Option::<String> {

    let input_file = path.to_str().unwrap_or_default().to_string();
    let content = read_content_from_file(&input_file);

    match content {
        Some(mut content) => {
            match parse(&mut content) {
                Some(tokens) => {
                    let html = generate_html(tokens, StyleType::Default, Pager::OnePage);
                    Some(html)
                },
                None => None,
            }
        },
        None => None,
    }
}