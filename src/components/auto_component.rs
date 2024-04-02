use dioxus::prelude::*;

use crate::nodes::bold::RenderBold;
use crate::nodes::document::RenderDocument;
use crate::nodes::heading::RenderHeading;
use crate::nodes::inline_code::RenderInlineCode;
use crate::nodes::italic::RenderItalic;
use crate::nodes::list::RenderList;
use crate::nodes::list_container::RenderListContainer;
use crate::nodes::list_item::RenderListItem;
use crate::nodes::paragraph::RenderParagraph;
use crate::nodes::paragraph_segment::RenderParagraphSegment;
use crate::nodes::spoiler::RenderSpoiler;
use crate::nodes::strikethrough::RenderStrikeThrough;
use crate::nodes::subscript::RenderSubscript;
use crate::nodes::superscript::RenderSuperscript;
use crate::nodes::underline::RenderUnderline;
use crate::nodes::NorgNode;

#[component]
pub fn AutoComponent(node: NorgNode) -> Element {
    match node {
        NorgNode::Document(doc) => rsx! {
            RenderDocument { node: doc }
        },
        NorgNode::Heading(heading) => rsx! {
            RenderHeading { node: heading }
        },
        NorgNode::Paragraph(p) => rsx! {
            RenderParagraph { node: p }
        },
        NorgNode::ParagraphSegment(ps) => rsx! {
            RenderParagraphSegment { node: ps }
        },
        NorgNode::ListContainer(lc) => rsx! {
            RenderListContainer { node: lc }
        },
        NorgNode::List(l) => rsx! {
            RenderList { node: l }
        },
        NorgNode::ListItem(li) => rsx! {
            RenderListItem { node: li }
        },
        NorgNode::Bold(b) => rsx! {
            RenderBold { node: b }
        },
        NorgNode::Italic(i) => rsx! {
            RenderItalic { node: i }
        },
        NorgNode::StrikeThrough(s) => rsx! {
            RenderStrikeThrough { node: s }
        },
        NorgNode::Underline(u) => rsx! {
            RenderUnderline { node: u }
        },
        NorgNode::Superscript(s) => rsx! {
            RenderSuperscript { node: s }
        },
        NorgNode::Subscript(s) => rsx! {
            RenderSubscript { node: s }
        },
        NorgNode::Spoiler(s) => rsx! {
            RenderSpoiler { node: s }
        },
        NorgNode::InlineCode(s) => rsx! {
            RenderInlineCode { node: s }
        },
    }
}
