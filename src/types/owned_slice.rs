use std::borrow::Cow;

use zerocopy::{AsBytes, FromBytes};

use crate::types::CowSlice;
use crate::{BytesDecode, BytesEncode};

/// Describes a [`Vec`] of types that are totally owned (doesn't
/// hold any reference to the original slice).
///
/// If you need to store a type that doesn't depends on any
/// [memory alignment] and that can be big it is recommended
/// to use the [`UnalignedSlice`].
///
/// The [`CowType`] is recommended for borrowed types (types that holds
/// references to the original slice).
///
/// [memory alignment]: https://doc.rust-lang.org/std/mem/fn.align_of.html
/// [`UnalignedSlice`]: crate::types::UnalignedSlice
/// [`CowType`]: crate::types::CowType
pub struct OwnedSlice<T>(std::marker::PhantomData<T>);

impl<'a, T: 'a> BytesEncode<'a> for OwnedSlice<T>
where
    T: AsBytes,
{
    type EItem = [T];

    fn bytes_encode(item: &'a Self::EItem) -> Option<Cow<[u8]>> {
        Some(Cow::Borrowed(<[T] as AsBytes>::as_bytes(item)))
    }
}

impl<'a, T: 'a> BytesDecode<'a> for OwnedSlice<T>
where
    T: FromBytes + Copy,
{
    type DItem = Vec<T>;

    fn bytes_decode(bytes: &[u8]) -> Option<Self::DItem> {
        CowSlice::<T>::bytes_decode(bytes).map(Cow::into_owned)
    }
}
