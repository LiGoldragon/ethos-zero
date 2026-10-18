//! Printing: an ethos file as the ordinary datom it is (proposal branch).
//!
//! The root's variant and the dot stand on the first line, the brace opens
//! the second, and each section follows on a line of its own from the first
//! column, the last closing the brace: the first struct of a file is not
//! indented. Each section is written by the protos writer, whose canonical
//! print expands vertically. A `Type` file's one declaration needs no break
//! and follows its dot on the same line.
//!
//! A vector of roots opens its bracket on the
//! first line; each root's head, dot and brace
//! stand together, its sections hang beneath it
//! two columns in, and the last closes the brace:
//!
//! ```text
//! [ Library.{
//!   []
//!   [ Name.String ]
//!   []
//!   [] }
//!   Memory.{
//!   []
//!   [ Record.{ Name Integer } ]
//!   []
//!   [] } ]
//! ```

use protos::{Enclosure, Protos, Protosizable, Separator, Textualizable};

use crate::{File, Printable};

/// The trait whose capability prints a root
/// standing in a vector of roots.
trait Hanging {
    /// Its head, dot and brace, then its
    /// sections two columns in.
    fn hang(&self) -> String;
}

/// The trait whose capability writes a section
/// on a new line, two columns in.
trait Indenting {
    fn indented(&self) -> String;
}

impl Indenting for Protos {
    fn indented(&self) -> String {
        let indent = "\n  ";
        let written = self.textualize();
        let lines = written.replace('\n', indent);
        format!("{indent}{lines}")
    }
}

impl Hanging for Protos {
    fn hang(&self) -> String {
        let Protos::Headed {
            head,
            constraints: None,
            separator: Separator::Period,
            body,
            ..
        } = self
        else {
            return self.textualize();
        };
        let Protos::Enclosed {
            enclosure: Enclosure::Braced,
            children,
            ..
        } = body.as_ref()
        else {
            return self.textualize();
        };
        let mut text = head.0.clone();
        text.push_str(".{");
        for section in children {
            text.push_str(&section.indented());
        }
        text.push_str(" }");
        text
    }
}

impl Printable for Protos {
    fn print(&self) -> String {
        let mut text = match self {
            Protos::Enclosed {
                enclosure: Enclosure::Bracketed,
                children,
                ..
            } if !children.is_empty() => {
                let mut text = "[ ".to_owned();
                let mut first = true;
                for root in children {
                    if !first {
                        text.push_str("\n  ");
                    }
                    first = false;
                    text.push_str(&root.hang());
                }
                text.push_str(" ]");
                text
            }
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
