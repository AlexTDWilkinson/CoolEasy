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

        page.push(html! {
            <div class="max-w-5xl mx-auto  px-8">
                <h1 class=" pt-12">"Welcome to CoolEasy!"</h1>
                
            {if !form_submitted {
                html! {<div  class="mt-8">"Here is a form for no reason other than to demonstrate using a form with CoolEasy."</div>}
            }
            else {"".to_string()}}

            {if form_idle {
                let form = form.clone();
                html! {
            <form action="" class="mt-4 flex flex-col gap-2" onsubmit="submitForm()">
                <input class="py-2 px-4 rounded-md border border-gray-900 max-w-fit" value={form.name.unwrap_or_default()} type="text" name="name" placeholder="Name"/>
                <input class="py-2 px-4 rounded-md border border-gray-900 max-w-fit" value={form.email.unwrap_or_default()} type="text" name="email" placeholder="Email"/>
                <button class="py-2 px-4 rounded-md border border-gray-900 max-w-fit disabled:bg-gray-500 disabled:cursor-not-allowed transition-colors" type="submit" id="submit-button">"Submit"</button>
                {if form_errors {
             
                    html! {
                        <div>
                            <ul class="text-red-800">
                                {validation_errors.unwrap().iter().map(|error| {
                                    html! {
                                   
                                                <li>{error.message.clone()}</li>
                                       
                                    }
                                }).collect::<Vec<_>>().join("")}
                            </ul>
                        </div>
                    }
                }
                else {"".to_string()}
            }
            </form>
                }
            }
            else {"".to_string()}
        }
            {if form_submitted {
                let form = form.clone();
                html! {
                    <div class="mt-12">
                        <h1>"Hello, "{form.name.unwrap_or_default()}"!"</h1>
                        <p>"Your email is "{form.email.unwrap_or_default()}"."</p>
                    </div>
                }
            }
            else {"".to_string()}
        }
        <p class="mt-12">"Here is a picture. You can put pictures and static files into the /public folder"</p> 

        <div class="mt-4 rounded-md overflow-hidden shadow-md max-w-sm">
        <img class="object-cover aspect-square" src="test.png"/>
        </div>

        <p class="mt-12">"Here is some stuff made into HTML from Markdown."</p>
        <p class="mt-2">"The styling of the markdown code blocks can easily altered with CSS."</p>

        <div class="mt-8 flex flex-col gap-8">
{md(
r#"
- Here is a markdown list
- With some items
- It doesn't really look like a list because you'd have to add some default styles for the UL and LI elements
"#)}
{md(
r#"
```rust
fn add() {
    let a = 5;
    let b = 10;
    let sum = a + b;
    println!("Sum of a and b = {}", sum);
}
```
"#)}
{md(
r#"
```tsx
interface MessageProps {
    text: string;
    important: boolean;
  }
   
  function Message({ text, important }: MessageProps) {
    return (
      <div>
        {important ? 'Important message: ' : 'Regular message: '}
        {text}
      </div>
    );
  }
```"#)}
        </div>

            <p class="mt-12 pb-16">"Here is a "<a href="/about" class="underline font-bold">"link"</a>" to the about page."</p>

        </div>
    });
 
    let html_content = PageWrapper::new()
        .title("Index")
        .description("Description")
        .add_css_style_sheet("style.css")
        .add_raw_header_injection(
            // cool old school JS to disable the button on form send
            "
            <script>
            function submitForm() {
              const submitButton = document.getElementById('submit-button');
              submitButton.disabled = true;
            }
          </script>
            "


        )
        .build(page);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html_content.into())?)
}
