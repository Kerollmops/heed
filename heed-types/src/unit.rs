use std::borrow::Cow;
use std::error::Error;
use heed_traits::{BytesDecode, BytesEncode};

/// Describes the `()` type.
pub struct Unit;

impl BytesEncode<'_> for Unit {
    type EItem = ();

    fn bytes_encode(_item: &Self::EItem) -> Result<Cow<[u8]>, Box<dyn Error>> {
        Ok(Cow::Borrowed(&[]))
    }
}

impl BytesDecode<'_> for Unit {
    type DItem = ();

    fn bytes_decode(bytes: &[u8]) -> Result<Self::DItem, Box<dyn Error>> {
        if bytes.is_empty() {
            Ok(())
        } else {
            Err("Bytes should be empty.")?
        }
    }
}
