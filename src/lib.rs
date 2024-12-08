use std::borrow::Cow;
use std::collections::TryReserveError;

macro_rules! count_digits {
  ($value:expr) => {{
    let mut value = $value;
    if value == 0 {
      return 1;
    }

    let mut count = 0;
    while value > 0 {
      value /= 10;
      count += 1;
    }
    count
  }};
}

macro_rules! impl_appendable_for_int {
  ($($t:ty),*) => {
    $(
      impl Appendable for $t {
        fn byte_len(&self) -> usize {
          count_digits!(*self)
        }

        fn push_to(&self, text: &mut String) {
          let mut buffer = itoa::Buffer::new();
          let s = buffer.format(*self);
          text.push_str(s);
        }
      }
    )*
  };
}

pub trait Appendable {
  fn byte_len(&self) -> usize;
  fn push_to(&self, text: &mut String);
}

impl Appendable for &str {
  fn byte_len(&self) -> usize {
    self.len()
  }

  fn push_to(&self, text: &mut String) {
    text.push_str(self);
  }
}

impl<'a> Appendable for &'a Cow<'a, str> {
  fn byte_len(&self) -> usize {
    self.len()
  }

  fn push_to(&self, text: &mut String) {
    text.push_str(self);
  }
}

impl_appendable_for_int!(
  i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl Appendable for char {
  fn byte_len(&self) -> usize {
    self.len_utf8()
  }

  fn push_to(&self, text: &mut String) {
    text.push(*self);
  }
}

impl<T: Appendable> Appendable for Option<T> {
  fn byte_len(&self) -> usize {
    match self {
      Some(value) => value.byte_len(),
      None => 0,
    }
  }

  fn push_to(&self, text: &mut String) {
    if let Some(value) = self {
      value.push_to(text);
    }
  }
}

impl<'a, T: Appendable + ?Sized> Appendable for &'a T {
  fn byte_len(&self) -> usize {
    (**self).byte_len()
  }

  fn push_to(&self, text: &mut String) {
    (**self).push_to(text)
  }
}

pub struct StringBuilder<'a> {
  capacity: usize,
  text: Option<String>,
  phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> StringBuilder<'a> {
  #[inline(always)]
  pub fn build(
    build: impl Fn(&mut StringBuilder<'a>),
  ) -> Result<String, TryReserveError> {
    let mut builder = StringBuilder {
      capacity: 0,
      text: None,
      phantom: std::marker::PhantomData,
    };
    build(&mut builder);
    let mut text = String::new();
    text.try_reserve(builder.capacity)?;
    builder.text = Some(String::with_capacity(builder.capacity));
    build(&mut builder);
    debug_assert_eq!(builder.capacity, builder.text.as_ref().unwrap().len());
    Ok(builder.text.unwrap())
  }

  pub fn append(&mut self, value: impl Appendable + 'a) {
    match &mut self.text {
      Some(t) => value.push_to(t),
      None => self.capacity += value.byte_len(),
    }
  }
}
