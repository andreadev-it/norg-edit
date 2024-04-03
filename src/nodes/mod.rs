use std::default::Default;
use std::fmt::Debug;

use anyhow::{Ok, Result};
use tree_sitter::Node;

pub mod bold;
pub mod document;
pub mod heading;
pub mod inline_code;
pub mod italic;
pub mod link;
pub mod list;
pub mod list_container;
pub mod list_item;
pub mod paragraph;
pub mod paragraph_segment;
pub mod spoiler;
pub mod strikethrough;
pub mod subscript;
pub mod superscript;
pub mod underline;

use self::{
    bold::Bold, document::Document, heading::Heading, inline_code::InlineCode, italic::Italic,
    link::Link, list::List, list_container::ListContainer, list_item::ListItem,
    paragraph::Paragraph, paragraph_segment::ParagraphSegment, spoiler::Spoiler,
    strikethrough::StrikeThrough, subscript::Subscript, superscript::Superscript,
    underline::Underline,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NorgNode {
    Document(Document),
    Heading(Heading),
    Paragraph(Paragraph),
    ParagraphSegment(ParagraphSegment),
    ListContainer(ListContainer),
    List(List),
    ListItem(ListItem),
    Link(Link),
    // ATTACHED MODIFIERS
    Bold(Bold),
    Italic(Italic),
    Underline(Underline),
    StrikeThrough(StrikeThrough),
    Spoiler(Spoiler),
    Superscript(Superscript),
    Subscript(Subscript),
    InlineCode(InlineCode),
    // NullModifier(NullModifier),
    // InlineMath(InlineMath),
    // Variable(Variable),
}

pub trait NodeItem {
    fn from_node<'a>(&mut self, node: Node<'a>, source: &str) -> Result<()>;
}

pub fn node_to_item<'a>(node: Node<'a>, source: &str) -> Result<NorgNode> {
    match node.kind() {
        "document" => {
            let mut doc = document::Document::default();
            doc.from_node(node, source)?;
            Ok(NorgNode::Document(doc))
        }
        "heading1" | "heading2" | "heading3" | "heading4" | "heading5" | "heading6" => {
            let mut heading = heading::Heading::default();
            heading.from_node(node, source)?;
            Ok(NorgNode::Heading(heading))
        }
        "list" => {
            let mut container = list_container::ListContainer::default();
            container.from_node(node, source)?;
            Ok(NorgNode::ListContainer(container))
        }
        "unordered_list_item1"
        | "unordered_list_item2"
        | "unordered_list_item3"
        | "unordered_list_item4"
        | "unordered_list_item5"
        | "unordered_list_item6" => {
            let mut list_item = list_item::ListItem::default();
            list_item.from_node(node, source)?;
            Ok(NorgNode::ListItem(list_item))
        }
        "ordered_list_item1" | "ordered_list_item2" | "ordered_list_item3"
        | "ordered_list_item4" | "ordered_list_item5" | "ordered_list_item6" => {
            let mut list_item = list_item::ListItem::default();
            list_item.from_node(node, source)?;
            Ok(NorgNode::ListItem(list_item))
        }
        "paragraph" => {
            let mut p = paragraph::Paragraph::default();
            p.from_node(node, source)?;
            Ok(NorgNode::Paragraph(p))
        }
        "paragraph_segment" => {
            let mut paragraph_segment = paragraph_segment::ParagraphSegment::default();
            paragraph_segment.from_node(node, source)?;
            Ok(NorgNode::ParagraphSegment(paragraph_segment))
        }
        "bold" => {
            let mut b = Bold::default();
            b.from_node(node, source)?;
            Ok(NorgNode::Bold(b))
        }
        "italic" => {
            let mut i = Italic::default();
            i.from_node(node, source)?;
            Ok(NorgNode::Italic(i))
        }
        "underline" => {
            let mut u = Underline::default();
            u.from_node(node, source)?;
            Ok(NorgNode::Underline(u))
        }
        "strikethrough" => {
            let mut s = StrikeThrough::default();
            s.from_node(node, source)?;
            Ok(NorgNode::StrikeThrough(s))
        }
        "superscript" => {
            let mut s = Superscript::default();
            s.from_node(node, source)?;
            Ok(NorgNode::Superscript(s))
        }
        "subscript" => {
            let mut s = Subscript::default();
            s.from_node(node, source)?;
            Ok(NorgNode::Subscript(s))
        }
        "spoiler" => {
            let mut s = Spoiler::default();
            s.from_node(node, source)?;
            Ok(NorgNode::Spoiler(s))
        }
        "verbatim" => {
            let mut c = InlineCode::default();
            c.from_node(node, source)?;
            Ok(NorgNode::InlineCode(c))
        }
        "link" => {
            let mut a = Link::default();
            a.from_node(node, source)?;
            Ok(NorgNode::Link(a))
        }
        _ => Err(anyhow::anyhow!("Unsupported node type: {}", node.kind())),
    }
}
