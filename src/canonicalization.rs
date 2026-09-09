//! Canonicalization: sweet Text to canonical Text (the mechanical conversion).
//!
//! An ethos file is written in the sweet form: the root's head, then the
//! sections as siblings. The reader sees only the braced form. The
//! conversion is mechanical and structural: the sweet text is
//! delineated, and when its first structure is a bare head, `.{` is
//! inserted right after it and a closing brace appended on its own
//! line. A text already in the braced form is left as it is.

use protos::{Extent, Protos, Protosizable};

use crate::{Canonical, Canonicalizable, Resituating};

impl Canonicalizable for String {
    fn canonicalize(&self) -> Result<Canonical, protos::Error> {
        let mut head_start = 0;
        let mut offset = 0;
        for line in self.split_inclusive('\n') {
            let trimmed = line.trim_start();
            if !trimmed.is_empty() && !trimmed.starts_with(';') {
                head_start = offset + (line.len() - trimmed.len());
                break;
            }
            offset += line.len();
        }
        let head_end = head_start
            + self[head_start..]
                .find(char::is_whitespace)
                .unwrap_or(self.len() - head_start);
        if self[head_start..head_end]
            .chars()
            .all(|glyph| glyph.is_ascii_alphabetic())
            && !self[head_end..].trim_start().starts_with(".{")
        {
            let mut text = String::with_capacity(self.len() + 4);
            text.push_str(&self[..head_end]);
            text.push_str(".{");
            text.push_str(&self[head_end..]);
            text.push_str("\n}");
            return Ok(Canonical {
                text,
                seam: Extent {
                    start: head_end,
                    end: head_end + 2,
                },
            });
        }
        let protos = <str as Protosizable>::protosize(self)?;
        if let Protos::Bare { extent, .. } = protos {
            let end = extent.end;
            let mut text = String::with_capacity(self.len() + 4);
            text.push_str(&self[..end]);
            text.push_str(".{");
            text.push_str(&self[end..]);
            text.push_str("\n}");
            return Ok(Canonical {
                text,
                seam: Extent {
                    start: end,
                    end: end + 2,
                },
            });
        }
        Ok(Canonical {
            text: self.clone(),
            seam: Extent { start: 0, end: 0 },
        })
    }
}

/// The kind whose capability maps one position across the seam.
trait Shifting {
    fn shift(&self, position: usize) -> usize;
}

impl Shifting for Canonical {
    fn shift(&self, position: usize) -> usize {
        let Extent { start, end } = self.seam;
        let inserted = end - start;
        let source_end = self.text.len() - 2 * inserted;
        if position <= start {
            position
        } else if position < end {
            start
        } else {
            (position - inserted).min(source_end)
        }
    }
}

impl Resituating for Canonical {
    fn resituate(&self, extent: Extent) -> Extent {
        Extent {
            start: self.shift(extent.start),
            end: self.shift(extent.end),
        }
    }
}
