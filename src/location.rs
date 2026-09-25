//! Location: an error situated in its source text as a line and a column.
//!
//! A structural error carries its extent in the source already. A
//! conceptual error carries a Protos path: the canonical text is read
//! again, the path is followed down its structure as far as the structure
//! goes, and the start of the node reached is carried back across the
//! seam into the source, where it is counted in lines and columns.

use datom_codec::Integer;
use protos::{Extent, Protos, Protosizable};

use crate::{Canonicalizable, Error, File, Locating, Location, Potential, Resituating};

/// The kind whose capability finds the extent of the node a path names.
trait Finding {
    fn find(&self, path: &[Integer]) -> Extent;
}

impl Finding for Protos {
    fn find(&self, path: &[Integer]) -> Extent {
        let Some((&step, rest)) = path.split_first() else {
            return self.whole();
        };
        match self {
            Protos::Headed {
                extent,
                head,
                constraints,
                body,
                ..
            } => match (step, constraints) {
                // A qualified head's arguments are children of the head.
                (0, Some(constraints)) if !rest.is_empty() => constraints.find(rest),
                (0, _) => Extent {
                    start: extent.start,
                    end: extent.start + head.0.len(),
                },
                (1, _) => body.find(rest),
                _ => *extent,
            },
            Protos::Enclosed {
                extent, children, ..
            } => match usize::try_from(step)
                .ok()
                .and_then(|index| children.get(index))
            {
                Some(child) => child.find(rest),
                None => *extent,
            },
            Protos::Opaque { extent, .. } | Protos::Bare { extent, .. } => *extent,
        }
    }
}

/// The kind whose capability yields a node's own extent.
trait Whole {
    fn whole(&self) -> Extent;
}

impl Whole for Protos {
    fn whole(&self) -> Extent {
        match self {
            Protos::Headed { extent, .. }
            | Protos::Enclosed { extent, .. }
            | Protos::Opaque { extent, .. }
            | Protos::Bare { extent, .. } => *extent,
        }
    }
}

/// The kind whose capability counts a byte offset of a text in lines and columns.
trait Lining {
    fn location(&self, offset: usize) -> Location;
}

impl Lining for str {
    fn location(&self, offset: usize) -> Location {
        let mut offset = offset.min(self.len());
        while !self.is_char_boundary(offset) {
            offset -= 1;
        }
        let before = &self[..offset];
        let line = before.matches('\n').count() + 1;
        let column = match before.rfind('\n') {
            Some(newline) => before[newline + 1..].chars().count() + 1,
            None => before.chars().count() + 1,
        };
        Location {
            line: line as Integer,
            column: column as Integer,
        }
    }
}

impl Locating for Potential<File> {
    fn locate(&self, error: &Error) -> Location {
        let offset = match error {
            Error::Structural(error) => error.extent.start,
            Error::Conceptual(data) => match self.0.canonicalize() {
                Ok(canonical) => match canonical.text.protosize() {
                    Ok(protos) => canonical.resituate(protos.find(&data.integer_vector)).start,
                    Err(_) => 0,
                },
                Err(_) => 0,
            },
        };
        self.0.location(offset)
    }
}
