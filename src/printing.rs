//! Printing: an ethos file as the ordinary datom it is (proposal branch).
//!
//! The root's variant and the dot stand on the first line, the brace opens
//! the second, and each section follows on a line of its own from the first
//! column, the last closing the brace: the first struct of a file is not
//! indented. Each section is written by the protos writer, whose canonical
//! print expands vertically. A `Type` file's one declaration needs no break
//! and follows its dot on the same line.

use protos::{Enclosure, Protos, Protosizable, Separator, Textualizable};

use crate::{File, Printable};

impl Printable for Protos {
    fn print(&self) -> String {
        let mut text = match self {
            Protos::Headed {
                head,
                constraints: None,
                separator: Separator::Period,
                body,
                ..
            } => match body.as_ref() {
                Protos::Enclosed {
                    enclosure: Enclosure::Braced,
                    children,
                    ..
                } if !children.is_empty() => {
                    let mut text = head.0.clone();
                    text.push_str(".\n{");
                    for section in children {
                        text.push('\n');
                        text.push_str(&section.textualize());
                    }
                    text.push_str(" }");
                    text
                }
                _ => self.textualize(),
            },
            _ => self.textualize(),
        };
        text.push('\n');
        text
    }
}

impl Printable for File {
    fn print(&self) -> String {
        self.protosize().print()
    }
}
