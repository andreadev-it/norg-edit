#[macro_export]
macro_rules! attached_modifier {
    ($x:ident) => {
        use crate::nodes::{node_to_item, NodeItem, NorgNode};
        use crate::utils::{get_node_splitted_text, Range};
        use anyhow::Result;
        use tree_sitter::Node;

        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct $x {
            text: Vec<String>,
            children: Vec<NorgNode>,
            pub textrange: Range,
        }

        impl NodeItem for $x {
            fn from_node<'a>(self: &mut Self, node: Node<'a>, source: &str) -> Result<()> {
                let mut cur = node.walk();

                self.children = node
                    .children(&mut cur)
                    .filter_map(|n| node_to_item(n, source).ok())
                    .collect();

                self.textrange = Range {
                    start: node.start_byte(),
                    end: node.end_byte(),
                };

                self.text = get_node_splitted_text(&self.textrange, &self.children, source, false);

                Ok(())
            }
        }
    };
}

pub(crate) use attached_modifier;
