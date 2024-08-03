pub fn render_markdown(source: &str) -> String {
    let parser = &mut markdown_it::MarkdownIt::new();

    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::extra::add(parser);

    let ast = parser.parse(source);

    return ast.render();
}
