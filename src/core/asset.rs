use crate::bindings;

pub struct Asset<T> {
  pub(crate) raw: *mut bindings::Asset<T>,
}
