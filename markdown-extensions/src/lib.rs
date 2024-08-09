use custom_blocks::CustomBlockDefinition;

mod custom_blocks;

#[derive(Debug)]
struct TextMessages;

impl CustomBlockDefinition for TextMessages {
    fn write_prefix(
        attrs: &std::collections::HashMap<String, String>,
        fmt: &mut dyn markdown_it::Renderer,
    ) {
        fmt.open(
            "div",
            &[(
                "class",
                format!("sms {}", if attrs.contains_key("me") { "me" } else { "" }),
            )],
        );
        fmt.open(
            "img",
            &[
                ("class", "sms-photo".to_string()),
                (
                    "src",
                    format!(
                        "/{}.svg",
                        attrs.get("image").map(String::as_str).unwrap_or("")
                    ),
                ),
            ],
        );
        fmt.open("div", &[]);
        fmt.open("div", &[("class", "sms-author".to_owned())]);
        fmt.text(attrs.get("name").map(String::as_str).unwrap_or(""));
        fmt.close("div");
        fmt.open("div", &[("class", "sms-content".to_owned())])
    }

    fn write_suffix(
        _attrs: &std::collections::HashMap<String, String>,
        fmt: &mut dyn markdown_it::Renderer,
    ) {
        fmt.close("div");
        fmt.close("div");
        fmt.close("div");
    }

    fn get_name() -> &'static str {
        "sms"
    }
}

pub fn render_markdown(source: &str) -> String {
    let parser = &mut markdown_it::MarkdownIt::new();

    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);
    custom_blocks::add::<TextMessages>(parser);

    let ast = parser.parse(source);

    return ast.render();
}
