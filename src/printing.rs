//! Printing: an ethos file in the sweet form. Its root's head stands on the
//! first line and each section on a line of its own, from the first column;
//! each section is written by the protos writer, whose canonical print
//! expands vertically. This pass places the sections and nothing else.

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
                } => {
                    let mut text = head.0.clone();
                    for section in children {
                        text.push('\n');
                        text.push_str(&section.textualize());
                    }
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
