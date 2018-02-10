macro_rules! readable {
  ($struct: ident, |$self:ident| $f: expr) => {
    impl<T> Readable<T> for $struct<T> {
      #[inline(always)]
      fn inner(&$self) -> *const T { $f }
    }
  };
}

macro_rules! writable {
  ($struct: ident, |$self:ident| $f: expr) => {
    impl<T> Writeable<T> for $struct<T> {
      #[inline(always)]
      fn inner(&mut $self) -> *mut T { $f }
    }
  };
}

macro_rules! readable_writable {
  ($struct: ident) => {
    impl<T> ReadableWriteable<T> for $struct<T>
      where T: ::core::ops::BitAnd<Output = T>,
            T: ::core::ops::BitOr<Output=T>{}
  };
}
