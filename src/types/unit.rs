use crate::{BytesDecode, BytesEncode};
use std::borrow::Cow;

/// Describes the [unit `()`] type.
///
/// [unit `()`]: https://doc.rust-lang.org/std/primitive.unit.html
pub struct Unit;

impl BytesEncode<'_> for Unit {
    type EItem = ();

    fn bytes_encode(_item: &Self::EItem) -> Option<Cow<[u8]>> {
        Some(Cow::Borrowed(&[]))
    }
}

impl BytesDecode<'_> for Unit {
    type DItem = ();

    fn bytes_decode(bytes: &[u8]) -> Option<Self::DItem> {
        if bytes.is_empty() {
            Some(())
        } else {
            None
        }
    }
}
