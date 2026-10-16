//! Canonicalization: an ethos text is already canonical.
//!
//! Proposal branch: an ethos file is an ordinary datom, its root's variant,
//! the dot, then the braced sections (`Library.\n{ … }`), or for a `Type`
//! file the one declaration (`Type.Name.{ … }`). The sweet form, a bare head
//! with the sections as its siblings, is no longer opened into braces; it
//! reads as more than one structure and is refused as `Multiple`. The seam
//! is therefore always empty, and resituating is the identity.

use protos::Extent;

use crate::{Canonical, Canonicalizable, Resituating};

impl Canonicalizable for String {
    fn canonicalize(&self) -> Result<Canonical, protos::Error> {
        Ok(Canonical {
            text: self.clone(),
            seam: Extent { start: 0, end: 0 },
        })
    }
}

/// The trait whose capability maps one position across the seam.
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
