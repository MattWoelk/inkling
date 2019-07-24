use crate::{
    line::{ChoiceData, Line, LineBuilder, ParsedLine},
    node::parse_root_node,
};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
/// Root of a single `Stitch`, containing all text and branching content belonging to it.
pub struct RootNode {
    pub items: Vec<NodeItem>,
    pub num_visited: u32,
}

impl RootNode {
    /// Parse a set of `ParsedLine` items and create a full graph representation of it.
    pub fn from_lines(lines: &[ParsedLine]) -> Self {
        parse_root_node(lines)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
/// Branch from a set of choices in a `Stitch`. Largely identical to `RootNode`
/// but also contains the data associated with the choice leading to it.
pub struct Branch {
    pub choice: ChoiceData,
    pub items: Vec<NodeItem>,
    pub num_visited: u32,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum NodeItem {
    Line(Line),
    BranchingChoice(Vec<Branch>),
}

#[cfg(test)]
/// Simplified checking of which match a `NodeItem` is during testing.
impl NodeItem {
    pub fn is_branching_choice(&self) -> bool {
        match self {
            NodeItem::BranchingChoice(..) => true,
            _ => false,
        }
    }

    pub fn is_line(&self) -> bool {
        match self {
            NodeItem::Line(..) => true,
            _ => false,
        }
    }
}

pub mod builders {
    use super::{Branch, ChoiceData, Line, LineBuilder, NodeItem, RootNode};

    /// Builder for a `RootNote`.
    ///
    /// # Notes
    ///  *  By default sets `num_visited` to 0.
    pub struct RootNodeBuilder {
        items: Vec<NodeItem>,
        num_visited: u32,
    }

    impl RootNodeBuilder {
        pub fn new() -> Self {
            RootNodeBuilder {
                items: Vec::new(),
                num_visited: 0,
            }
        }

        pub fn build(self) -> RootNode {
            RootNode {
                items: self.items,
                num_visited: self.num_visited,
            }
        }

        pub fn add_branching_choice(&mut self, branching_set: Vec<Branch>) {
            self.add_item(NodeItem::BranchingChoice(branching_set));
        }

        pub fn add_item(&mut self, item: NodeItem) {
            self.items.push(item);
        }

        pub fn add_line(&mut self, line: Line) {
            self.add_item(NodeItem::Line(line));
        }

        #[cfg(test)]
        pub fn with_branching_choice(mut self, branching_choice_set: NodeItem) -> Self {
            self.items.push(branching_choice_set);
            self
        }

        #[cfg(test)]
        pub fn with_line_text(mut self, content: &str) -> Self {
            let line = NodeItem::Line(LineBuilder::new().with_text(content).unwrap().build());
            self.items.push(line);
            self
        }
    }

    /// Builder for a `Branch`, created from a `ChoiceData` that spawns the branch in
    /// the parsed lines of text content.
    ///
    /// # Notes
    ///  *  Adds the line from its choice as the first in its item list.
    pub struct BranchBuilder {
        choice: ChoiceData,
        items: Vec<NodeItem>,
        num_visited: u32,
    }

    impl BranchBuilder {
        pub fn from_choice(choice: ChoiceData) -> Self {
            let line = LineBuilder::new().with_line(choice.line.clone()).build();

            BranchBuilder {
                choice,
                items: vec![NodeItem::Line(line)],
                num_visited: 0,
            }
        }

        pub fn add_branching_choice(&mut self, branching_set: Vec<Branch>) {
            self.add_item(NodeItem::BranchingChoice(branching_set));
        }

        pub fn add_item(&mut self, item: NodeItem) {
            self.items.push(item);
        }

        pub fn add_line(&mut self, line: Line) {
            self.add_item(NodeItem::Line(line));
        }

        #[cfg(test)]
        pub fn with_branching_choice(mut self, branching_choice_set: NodeItem) -> Self {
            self.items.push(branching_choice_set);
            self
        }

        #[cfg(test)]
        pub fn with_line_text(mut self, content: &str) -> Self {
            let line = NodeItem::Line(LineBuilder::new().with_text(content).unwrap().build());
            self.items.push(line);
            self
        }

        pub fn build(self) -> Branch {
            Branch {
                choice: self.choice,
                items: self.items,
                num_visited: self.num_visited,
            }
        }
    }

    #[cfg(test)]
    pub struct BranchingChoiceBuilder {
        items: Vec<Branch>,
    }

    #[cfg(test)]
    impl BranchingChoiceBuilder {
        pub fn new() -> Self {
            BranchingChoiceBuilder { items: Vec::new() }
        }

        pub fn with_branch(mut self, choice: Branch) -> Self {
            self.items.push(choice);
            self
        }

        pub fn build(self) -> NodeItem {
            NodeItem::BranchingChoice(self.items)
        }
    }
}
