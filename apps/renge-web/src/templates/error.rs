use askama::Template;

#[derive(Template)]
#[template(path = "error.askama.html")]
pub(crate) struct ErrorTemplate {
    pub(crate) message: String,
}
