//! Printing: the canonical print, which expands vertically.
//!
//! A structure with more than one element, one of which has a next layer,
//! opens on its line and its elements hang beneath the first, aligned; the
//! closing delimiter ends the last element's line. Elements that are all
//! leaves sit on one line. A space stands inside every non-empty bracket
//! and brace; an empty one is `[]` or `{}`. An angled enclosure is the
//! arguments of the element before it and stays tight against it, on one
//! line, its elements one space apart. A file prints in the sweet form: its root's head on the first
//! line, each section on a line of its own, from the first column.
//!
//! An element is a node and the angled enclosures that follow it. It has a
//! next layer when its node is headed or an enclosure; a bare name, with
//! its arguments, is a leaf. Every leaf, and every empty enclosure, is
//! written by the protos writer; this pass lays out the rest.

use protos::{Enclosure, Protos, Protosizable, Separator, Textualizable};

use crate::{File, Printable};

/// The kind whose capability yields a delimiter's or separator's glyph.
trait Glyphing {
    fn glyph(&self) -> char;
}

impl Glyphing for Separator {
    fn glyph(&self) -> char {
        match self {
            Separator::Period => '.',
            Separator::Exclamation => '!',
            Separator::Colon => ':',
        }
    }
}

/// The kind whose capabilities yield an enclosure's opening and closing glyphs.
trait Delimiting {
    fn opener(&self) -> char;
    fn closer(&self) -> char;
}

impl Delimiting for Enclosure {
    fn opener(&self) -> char {
        match self {
            Enclosure::Braced => '{',
            Enclosure::Bracketed => '[',
            Enclosure::Angled => '<',
        }
    }
    fn closer(&self) -> char {
        match self {
            Enclosure::Braced => '}',
            Enclosure::Bracketed => ']',
            Enclosure::Angled => '>',
        }
    }
}

/// The text being laid out, and where its current line stands.
struct Page {
    text: String,
}

/// The kind whose capabilities write onto a page.
trait Writing {
    fn column(&self) -> usize;
    fn write(&mut self, text: &str);
    fn hang(&mut self, column: usize);
}

impl Writing for Page {
    fn column(&self) -> usize {
        let start = self.text.rfind('\n').map_or(0, |at| at + 1);
        self.text[start..].chars().count()
    }
    fn write(&mut self, text: &str) {
        self.text.push_str(text);
    }
    fn hang(&mut self, column: usize) {
        self.text.push('\n');
        self.text.extend(std::iter::repeat_n(' ', column));
    }
}

/// The kind whose capability lays a node out onto a page.
trait Laying {
    fn lay(&self, page: &mut Page);
}

/// The kind whose capability says whether an element has a next layer.
trait Layered {
    fn layered(&self) -> bool;
}

impl Layered for [&Protos] {
    fn layered(&self) -> bool {
        matches!(
            self.first(),
            Some(Protos::Headed { .. })
                | Some(Protos::Enclosed {
                    enclosure: Enclosure::Braced | Enclosure::Bracketed,
                    ..
                })
        )
    }
}

/// The kind whose capability groups enclosed children into elements: each
/// node with the angled enclosures that follow it.
trait Grouping {
    fn elements(&self) -> Vec<Vec<&Protos>>;
}

impl Grouping for [Protos] {
    fn elements(&self) -> Vec<Vec<&Protos>> {
        let mut elements: Vec<Vec<&Protos>> = Vec::new();
        for child in self {
            let attached = matches!(
                child,
                Protos::Enclosed {
                    enclosure: Enclosure::Angled,
                    ..
                }
            );
            match elements.last_mut() {
                Some(element) if attached => element.push(child),
                _ => elements.push(vec![child]),
            }
        }
        elements
    }
}

impl Laying for Protos {
    fn lay(&self, page: &mut Page) {
        match self {
            Protos::Headed {
                head,
                constraints,
                separator,
                body,
                ..
            } => {
                page.write(&head.0);
                if let Some(constraints) = constraints {
                    constraints.lay(page);
                }
                page.write(separator.glyph().encode_utf8(&mut [0; 4]));
                body.lay(page);
            }
            Protos::Enclosed {
                enclosure: enclosure @ (Enclosure::Braced | Enclosure::Bracketed),
                children,
                ..
            } if !children.is_empty() => {
                let elements = children.elements();
                let vertical =
                    elements.len() > 1 && elements.iter().any(|element| element.layered());
                page.write(enclosure.opener().encode_utf8(&mut [0; 4]));
                page.write(" ");
                let column = page.column();
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        if vertical {
                            page.hang(column);
                        } else {
                            page.write(" ");
                        }
                    }
                    for node in element {
                        node.lay(page);
                    }
                }
                page.write(" ");
                page.write(enclosure.closer().encode_utf8(&mut [0; 4]));
            }
            Protos::Enclosed {
                enclosure: Enclosure::Angled,
                children,
                ..
            } => {
                page.write("<");
                for (index, element) in children.elements().iter().enumerate() {
                    if index > 0 {
                        page.write(" ");
                    }
                    for node in element {
                        node.lay(page);
                    }
                }
                page.write(">");
            }
            Protos::Enclosed { .. } | Protos::Bare { .. } | Protos::Opaque { .. } => {
                page.write(&self.textualize());
            }
        }
    }
}

impl Printable for Protos {
    fn print(&self) -> String {
        let mut page = Page {
            text: String::new(),
        };
        match self {
            Protos::Headed {
                head,
                constraints: None,
                separator: Separator::Period,
                body,
                ..
            } if matches!(
                body.as_ref(),
                Protos::Enclosed {
                    enclosure: Enclosure::Braced,
                    ..
                }
            ) =>
            {
                page.write(&head.0);
                if let Protos::Enclosed { children, .. } = body.as_ref() {
                    for section in children {
                        page.hang(0);
                        section.lay(&mut page);
                    }
                }
            }
            _ => self.lay(&mut page),
        }
        page.write("\n");
        page.text
    }
}

impl Printable for File {
    fn print(&self) -> String {
        self.protosize().print()
    }
}
