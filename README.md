# CoolEasy Rust Web Framework

CoolEasy is a Rust web framework that enables you to create and deploy a website to the "edge" in under 5 minutes.

Its primary goal is to promote more Rust-powered sites on the internet and provide Rust developers with a simple and enjoyable way to dive into web development. While Rust offers many outstanding web frameworks, CoolEasy fills a niche for those who want to rapidly launch a simple website or blog without any hassle, and for free.

Designed for speed and ease of use, CoolEasy enables the creation of server-rendered websites and seamless deployment to Vercel, with each page becoming a Vercel edge function. The framework offers an intuitive and expressive API for building web pages, supporting both HTML and Markdown, as well as providing out-of-the-box markdown code colorization.

By default, CoolEasy utilizes Tailwind CSS for styling, although it's not mandatory. If you prefer, you can edit the style.css file manually. Ultimately, CoolEasy is all about creating and deploying websites with Rust in a fast and efficient manner.

## Features

- Create webpages quickly using handy macros like html! and md!
- Tailwind CSS is used by default for styling
- Deploy your website to Vercel Lambdas for easy scaling and performance
- Leverage the power of server-side rendering for fast page loads and great SEO
- Use the browser - HTML 5, forms, and regular old JavaScript provide interactivity.
- Hot reloading - you don't have to recompile the full Rust project on every change.

## Example Usage

There is a basic example in the examples folder.

Here is the basic example deployed: https://portfolio-houski.vercel.app/

The following code shows a typical page in the CoolEasy framework:

```rust
// ... (imports and struct definitions)

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // ... (any form processing and validation)

    let mut page = vec![];

    // If the page is interactive, often this will conditional rendering based on form submissions (see basic example for details)

    page.push(html! {
    <div>
        <h1>"Hello, world!"</h1>
    </div>
    })

    page.push(md!("## I am markdown"));


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
```

To start the framework, use the command:

`npx vercel dev`

If you want to use Tailwind to style your website, run:

`npx tailwindcss -i ./input.css -o ./public/style.css --watch`

To hot reload the website run:

`npx browser-sync start --proxy "localhost:3000" --files "**/*.*"` (this should target specific files, I just haven't figured out a good glob pattern yet and this works, so...)

To deploy your website, simply run:

`npx vercel`

## What is Cool Easy good at?

- Portfolios
- Blogs
- Landing pages
- Small business websites
- Documentation websites
- Event websites

## What is Cool Easy bad at?

- Highly interactive sites with complex realtime UI interactions
- Deploying to anywhere except Vercel (though maybe in the future that will change)
