use cool_easy::{html, md, PageWrapper};
use portfolio::CoolEasyValidationError;
use serde::{Deserialize, Serialize};
use validator::Validate;

use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Default, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate)]
pub struct IndexForm {
    #[validate(
        required(message = "Name is required"),
        length(min = 3, message = "Name is too short")
    )]
    pub name: Option<String>,
    #[validate(
        required(message = "Email is required"),
        email(message = "Invalid email")
    )]
    pub email: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let form: IndexForm = serde_qs::from_str(req.uri().query().unwrap_or("")).unwrap_or_default();

    println!("form: {:?}", form);

    let validation_errors = form.validate();

    let validation_errors = match validation_errors {
        Ok(_) => None,
        Err(e) => Some(
            e.field_errors()
                .iter()
                .map(|(field, errors)| CoolEasyValidationError {
                    field: field.to_string(),
                    message: errors
                        .first()
                        .map(|error| error.to_string())
                        .unwrap_or_default(),
                })
                .collect::<Vec<_>>(),
        ),
    };

    let form_idle = validation_errors.is_some();
    let form_submitted = validation_errors.is_none();
    let form_errors = validation_errors.is_some() && form != IndexForm::default();

    let mut page = vec![];


        let form = form.clone();
        page.push(html! {
            <div class="max-w-4xl mx-auto mt-8">
            <div>
                <h1 class="mt-2">"About"</h1>
            </div>

            <div  class="mt-12">"Glad you could make it."</div>

            <p class="mt-12 mb-16">"Here is a "<a href="/" class="underline font-bold">"link"</a>" to the homepage."</p>
           

            </div>
       


        });



 

    let html_content = PageWrapper::new()
        .title("Index")
        .description("Description")
        .add_css_style_sheet("style.css")
        .build(page);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html_content.into())?)
}
