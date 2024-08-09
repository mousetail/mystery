use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

pub trait CustomBlockDefinition: Debug {
    fn write_prefix(attrs: &HashMap<String, String>, fmt: &mut dyn Renderer);
    fn write_suffix(attrs: &HashMap<String, String>, fmt: &mut dyn Renderer);
    fn get_name() -> &'static str;
}

#[derive(Debug)]
/// AST node for front-matter
pub struct CustomBlock<Definition: CustomBlockDefinition> {
    attributes: HashMap<String, String>,
    phantom: PhantomData<Definition>,
}

impl<Definition: CustomBlockDefinition + 'static> NodeValue for CustomBlock<Definition> {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        Definition::write_prefix(&self.attributes, fmt);

        fmt.cr();

        fmt.contents(&node.children);

        fmt.cr();

        Definition::write_suffix(&self.attributes, fmt);

        fmt.cr();
    }
}

/// Add the front-matter extension to the markdown parser
pub fn add<Definition: CustomBlockDefinition + 'static>(md: &mut MarkdownIt) {
    // insert this rule into block subparser
    md.block
        .add_rule::<CustomBlockBlockScanner<Definition>>()
        .before_all();
}

/// An extension for the block subparser.
struct CustomBlockBlockScanner<Definition: CustomBlockDefinition>(PhantomData<Definition>);

fn parse_attributes(line: &str, name: &str) -> Option<HashMap<String, String>> {
    let mut words = line.split(" ");
    (words.next() == Some(name)).then(|| {
        let mut out = HashMap::new();

        for word in words {
            if let Some((key, value)) = word.split_once(':') {
                out.insert(key.to_string(), value.to_string());
            } else {
                out.insert(word.to_string(), "".to_string());
            }
        }

        return out;
    })
}

fn parse_markdown_in_line_range(
    state: &mut BlockState,
    start_line: usize,
    end_line: usize,
    parent_node: Node,
) -> Node {
    let old_indent = state.blk_indent;
    state.blk_indent = 0;

    let old_node = std::mem::replace(&mut state.node, parent_node);
    let old_line_max = state.line_max;
    state.line = start_line;
    state.line_max = end_line;
    state.md.block.tokenize(state);
    state.line = start_line;
    state.line_max = old_line_max;

    state.blk_indent = old_indent;

    return std::mem::replace(&mut state.node, old_node);
}

impl<Definition: CustomBlockDefinition + 'static> BlockRule
    for CustomBlockBlockScanner<Definition>
{
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // check line starts with opening dashes
        let opening = state
            .get_line(state.line)
            .chars()
            .take_while(|c| *c == ':')
            .collect::<String>();
        if !opening.starts_with(":::") {
            return None;
        }

        let attributes = parse_attributes(
            &state.get_line(state.line)[opening.len()..].trim(),
            Definition::get_name(),
        )?;

        // Search for the end of the block
        let mut next_line = state.line;
        loop {
            next_line += 1;
            if next_line >= state.line_max {
                return None;
            }

            let line = state.get_line(next_line);
            if line.starts_with(&opening) {
                break;
            }
        }

        let mut node = Node::new(CustomBlock::<Definition> {
            attributes,
            phantom: PhantomData,
        });
        let start_line = state.line;
        node = parse_markdown_in_line_range(state, start_line + 1, next_line, node);
        // return new node and number of lines it occupies
        Some((node, next_line - state.line + 1))
    }
}
