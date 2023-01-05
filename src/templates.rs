use askama::Template; // bring trait in scope
pub use axum_core::response::{IntoResponse, Response};
use http::StatusCode;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => {
                let headers = [(
                    http::header::CONTENT_TYPE,
                    http::HeaderValue::from_static(T::MIME_TYPE),
                )];

                (headers, html).into_response()
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}




#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
pub struct HelloTemplate { // the name of the struct can be anything
    pub name: String, // the field name should match the variable name
}



