//! Printing: the canonical braced form, with the first section after
//! the root and subsequent sections aligned beneath it. Protos owns
//! every delimiter and layout decision.

use protos::{Protos, Protosizable, Textualizable};

use crate::{File, Printable};

impl Printable for Protos {
    fn print(&self) -> String {
        let mut text = self.textualize();
        text.push('\n');
        text
    }
}

impl Printable for File {
    fn print(&self) -> String {
        self.protosize().print()
    }
}
