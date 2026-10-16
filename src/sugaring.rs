//! Sugaring (proposal): `⟦ T ⟧` is `Vector<T>` in a type position.
//!
//! Protos reads `⟦ ⟧` as an enclosure of its own. Before conception, every
//! vector bracket standing in a list, alone or as the innermost body of a
//! headed element (`Locks.⟦ Lock ⟧`), is rewritten into the two siblings
//! `Vector<Integer>` reads as: a bare `Vector`, at the opening bracket, and
//! an angled enclosure of the bracket's children, at the whole bracket. The
//! rest of the reader, its spans and its error paths, then sees the form it
//! already knows. A vector bracket with no list around it, the body of a
//! `Type.` file's alias, is read where a reference is (`conception`).

use protos::{Enclosure, Extent, Protos};

/// The trait whose capability rewrites the vector brackets below a node.
pub(crate) trait Sugaring {
    /// The same tree, its listed vector brackets spelled `Vector<…>`.
    fn desugar(&self) -> Protos;
}

/// The trait whose capability rewrites the children of a list, each split in place.
trait SugaringEach {
    fn desugar_each(&self) -> Vec<Protos>;
}

impl SugaringEach for [Protos] {
    fn desugar_each(&self) -> Vec<Protos> {
        let mut desugared = Vec::with_capacity(self.len());
        for child in self {
            desugared.extend(child.split());
        }
        desugared
    }
}

/// The trait whose capability splits one listed element into the siblings it stands for.
trait Splitting {
    /// The element itself, or the element ending in a bare `Vector` and the
    /// angled enclosure beside it.
    fn split(&self) -> Vec<Protos>;
}

impl Sugaring for Protos {
    fn desugar(&self) -> Protos {
        match self {
            Protos::Headed {
                extent,
                head,
                constraints,
                separator,
                body,
            } => Protos::Headed {
                extent: *extent,
                head: head.clone(),
                constraints: constraints.clone(),
                separator: *separator,
                body: Box::new(body.desugar()),
            },
            Protos::Enclosed {
                extent,
                enclosure,
                children,
            } => Protos::Enclosed {
                extent: *extent,
                enclosure: *enclosure,
                children: children.desugar_each(),
            },
            other => other.clone(),
        }
    }
}

impl Splitting for Protos {
    fn split(&self) -> Vec<Protos> {
        match self {
            Protos::Enclosed {
                extent,
                enclosure: Enclosure::Vectored,
                children,
            } => vec![
                Protos::Bare {
                    extent: Extent {
                        start: extent.start,
                        end: extent.start + '⟦'.len_utf8(),
                    },
                    text: String::from("Vector"),
                },
                Protos::Enclosed {
                    extent: *extent,
                    enclosure: Enclosure::Angled,
                    children: children.desugar_each(),
                },
            ],
            Protos::Headed {
                extent,
                head,
                constraints,
                separator,
                body,
            } => {
                let mut parts = body.split();
                let first = parts.remove(0);
                let mut split = vec![Protos::Headed {
                    extent: *extent,
                    head: head.clone(),
                    constraints: constraints.clone(),
                    separator: *separator,
                    body: Box::new(first),
                }];
                split.extend(parts);
                split
            }
            other => vec![other.desugar()],
        }
    }
}
