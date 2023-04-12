pub use cool_easy_macros::html;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn md(markdown: &str) -> String {
    use femark::{process_markdown_to_html, HTMLOutput};
    match process_markdown_to_html(markdown.to_owned()) {
        Ok(HTMLOutput { content, toc: _ }) => content,
        Err(_e) => "".to_string(),
    }
}

pub struct PageWrapper {
    title: Option<String>,
    description: Option<String>,
    css_style_sheets: Vec<String>,
    javascript_header_tags: Vec<String>,
    javascript_body_end_tags: Vec<String>,
    raw_header_injection: Option<String>,
    raw_body_end_injection: Option<String>,
}

impl PageWrapper {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            css_style_sheets: vec![],
            javascript_header_tags: vec![],
            javascript_body_end_tags: vec![],
            raw_header_injection: None,
            raw_body_end_injection: None,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn add_css_style_sheet(mut self, css_style_sheet: &str) -> Self {
        self.css_style_sheets.push(css_style_sheet.to_string());
        self
    }

    pub fn add_javascript_tag_to_head(mut self, script_tag: &str) -> Self {
        self.javascript_header_tags.push(script_tag.to_string());
        self
    }

    pub fn add_javascript_tag_to_end_of_body(mut self, script_tag: &str) -> Self {
        self.javascript_body_end_tags.push(script_tag.to_string());
        self
    }

    pub fn add_raw_header_injection(mut self, raw_header_injection: &str) -> Self {
        self.raw_header_injection = Some(raw_header_injection.to_string());
        self
    }

    pub fn add_raw_body_end_injection(mut self, raw_body_end_injection: &str) -> Self {
        self.raw_body_end_injection = Some(raw_body_end_injection.to_string());
        self
    }

    pub fn build(self, page: Vec<String>) -> String {
        let mut page_wrapper: Vec<String> = vec![];

        // do HTML and doc type
        page_wrapper.push(String::from("<!DOCTYPE html>"));
        page_wrapper.push(String::from("<html>"));

        page_wrapper.push(String::from("<head>"));

        if let Some(title) = self.title {
            page_wrapper.push(html! {
                    <title>{title}</title>
            });
        }

        if let Some(description) = self.description {
            page_wrapper.push(html! {
                    <meta name="description" content={description} />
            });
        }

        if let Some(raw_header_injection) = self.raw_header_injection {
            page_wrapper.push(raw_header_injection);
        }

        for css_style_sheet in self.css_style_sheets {
            page_wrapper.push(html! {
                    <link rel="stylesheet" href={css_style_sheet} />
            });
        }

        for javascript_header_tag in self.javascript_header_tags {
            page_wrapper.push(html! {
                    <script src={javascript_header_tag}></script>
            });
        }

        page_wrapper.push(String::from("</head>"));

        page_wrapper.push(String::from("<body>"));

        for section in page {
            page_wrapper.push(section);
        }

        if let Some(raw_body_end_injection) = self.raw_body_end_injection {
            page_wrapper.push(raw_body_end_injection);
        }

        for javascript_body_end_tag in self.javascript_body_end_tags {
            page_wrapper.push(html! {
                    <script src={javascript_body_end_tag}></script>
            });
        }

        page_wrapper.push(String::from("</body>"));

        page_wrapper.push(String::from("</html>"));

        return page_wrapper.join("");
    }
}
