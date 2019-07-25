use crate::line::{Alternative, Condition};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
/// Representation of a single line of Ink content. All of its raw data will be processed
/// into a final form and presented to the user as the story is followed.
pub struct FullLine {
    /// Root chunk of line content, which may possibly be nested into even finer parts.
    pub chunk: LineChunk,
    /// Tags associated with the line. Will be given to the user along with the processed
    /// line content as the story is followed.
    pub tags: Vec<String>,
    /// Whether or not the line is glued to the previous line. Glue prohibits new lines
    /// to be added between lines, which is otherwise the default behavior when following
    /// the story.
    pub glue_begin: bool,
    /// Whether or not the line is glued to the next line.
    pub glue_end: bool,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
/// Line content is nested into these smaller chunks. When the chunk is processed
/// it will, in order, process all child items. The simplest example is a line
/// of text with a divert. This can be represented as two items in this chunk.
/// When the chunk is processed the line content will be visited first, then
/// the divert will be encountered and returned through the call stack.
///
/// Chunks possibly come with conditions for when the content will be visited
/// and displayed to the user.
pub struct LineChunk {
    pub conditions: Vec<Condition>,
    pub items: Vec<Content>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
/// Items in each chunk of line content comes in these forms.
pub enum Content {
    /// Content that alternates every time it is visited in the story.
    Alternative(Alternative),
    /// Divert to a new node in the story.
    Divert(String),
    /// Null content.
    Empty,
    /// String of regular text content in the line.
    Text(String),
}

impl FullLine {
    pub fn from_chunk(chunk: LineChunk) -> Self {
        FullLine {
            chunk,
            tags: Vec::new(),
            glue_begin: false,
            glue_end: false,
        }
    }

    pub fn text(&self) -> String {
        let mut buffer = String::new();

        for item in &self.chunk.items {
            match item {
                Content::Text(string) => {
                    buffer.push_str(&string);
                }
                _ => (),
            }
        }

        buffer
    }

    #[cfg(test)]
    pub fn from_string(line: &str) -> Self {
        use builders::LineChunkBuilder;

        let chunk = LineChunkBuilder::from_string(line).build();
        Self::from_chunk(chunk)
    }
}

pub mod builders {
    use super::*;

    pub struct FullLineBuilder {
        chunk: LineChunk,
        tags: Vec<String>,
        glue_begin: bool,
        glue_end: bool,
    }

    impl FullLineBuilder {
        pub fn from_chunk(chunk: LineChunk) -> Self {
            FullLineBuilder {
                chunk,
                tags: Vec::new(),
                glue_begin: false,
                glue_end: false,
            }
        }

        pub fn build(self) -> FullLine {
            FullLine {
                chunk: self.chunk,
                tags: self.tags,
                glue_begin: self.glue_begin,
                glue_end: self.glue_end,
            }
        }

        pub fn set_divert(&mut self, address: &str) {
            self.chunk.items.push(Content::Divert(address.to_string()));
        }

        pub fn set_glue_begin(&mut self, glue: bool) {
            self.glue_begin = glue;
        }

        pub fn set_glue_end(&mut self, glue: bool) {
            self.glue_end = glue;
        }

        pub fn set_tags(&mut self, tags: &[String]) {
            self.tags = tags.to_vec();
        }

        #[cfg(test)]
        pub fn from_string(line: &str) -> Self {
            Self::from_chunk(LineChunkBuilder::from_string(line).build())
        }

        #[cfg(test)]
        pub fn with_glue_begin(mut self) -> Self {
            self.glue_begin = true;
            self
        }

        #[cfg(test)]
        pub fn with_glue_end(mut self) -> Self {
            self.glue_end = true;
            self
        }

        #[cfg(test)]
        pub fn with_tags(mut self, tags: &[String]) -> Self {
            self.set_tags(tags);
            self
        }
    }

    pub struct LineChunkBuilder {
        items: Vec<Content>,
    }

    impl LineChunkBuilder {
        pub fn new() -> Self {
            LineChunkBuilder { items: Vec::new() }
        }

        pub fn build(self) -> LineChunk {
            LineChunk {
                conditions: Vec::new(),
                items: self.items,
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.add_item(Content::Text(text.to_string()));
        }

        pub fn add_divert(&mut self, address: &str) {
            self.add_item(Content::Divert(address.to_string()));
        }

        pub fn add_item(&mut self, item: Content) {
            self.items.push(item);
        }

        #[cfg(test)]
        pub fn with_divert(self, address: &str) -> Self {
            self.with_item(Content::Divert(address.to_string()))
        }

        #[cfg(test)]
        pub fn with_item(mut self, item: Content) -> Self {
            self.items.push(item);
            self
        }

        #[cfg(test)]
        pub fn with_text(self, text: &str) -> Self {
            self.with_item(Content::Text(text.to_string()))
        }

        #[cfg(test)]
        pub fn from_string(line: &str) -> Self {
            LineChunkBuilder::new().with_text(line)
        }
    }
}