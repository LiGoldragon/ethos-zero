//! Printing: an ethos file in the sweet form. Its root's head stands on the
//! first line and each section on a line of its own, from the first column;
//! each section is written by the protos writer, whose canonical print
//! expands vertically. Printing places those sections; checking is separate.

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

/// The trait whose capability removes only canonical padding immediately
/// inside typed white-bracket vector delimiters. Opaque text and
/// parenthesized retained text are left untouched because they are not
/// syntax nodes.
trait Tightening {
    fn tighten_vectors(&self) -> String;
}

impl Tightening for str {
    fn tighten_vectors(&self) -> String {
        let mut output = String::with_capacity(self.len());
        let mut opaque = None;
        let mut escaped = false;
        let mut chars = self.chars().peekable();
        while let Some(character) = chars.next() {
            if let Some(closer) = opaque {
                output.push(character);
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == closer {
                    opaque = None;
                }
                continue;
            }
            match character {
                '«' => {
                    opaque = Some('»');
                    output.push(character);
                }
                '(' => {
                    opaque = Some(')');
                    output.push(character);
                }
                '⟦' => {
                    output.push(character);
                    if chars.peek() == Some(&' ') {
                        chars.next();
                    }
                }
                '⟧' => {
                    if output.ends_with(' ') {
                        output.pop();
                    }
                    output.push(character);
                }
                _ => output.push(character),
            }
        }
        output
    }
}

impl Printable for File {
    fn print(&self) -> String {
        self.protosize().print().tighten_vectors()
    }
}

#[cfg(test)]
mod tests {
    use super::Tightening;

    #[test]
    fn opaque_text_is_not_rewritten_while_vectors_are_tightened() {
        assert_eq!(
            "«keep ⟦ spaces ⟧» ⟦ X ⟧ (keep ⟦ spaces ⟧)".tighten_vectors(),
            "«keep ⟦ spaces ⟧» ⟦X⟧ (keep ⟦ spaces ⟧)"
        );
    }
}
