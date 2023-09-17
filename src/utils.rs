use libffi::high::Closure1;

pub fn closure_to_extern_fn<F: Fn() + Send + Sync + 'static>(f: F) -> unsafe extern "C" fn() {
  let closure: &'static _ = Box::leak(Box::new(move |_: ()| f()));
  let callback = Closure1::new(closure);
  let &code = callback.code_ptr();
  let ptr: unsafe extern "C" fn() = unsafe { std::mem::transmute(code) };
  std::mem::forget(callback);

  ptr
}
