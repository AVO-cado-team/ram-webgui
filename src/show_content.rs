use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, HtmlElement};

// This shit should not go to production
pub fn show_content() -> Result<(), ()> {
  let window = web_sys::window().ok_or(())?;
  let document: Document = window.document().ok_or(())?;
  let main = document.get_element_by_id("ram-web").ok_or(())?;
  let loader = document.get_element_by_id("loader").ok_or(())?;

  let counter = Rc::new(RefCell::new(0.0));

  let closure = {
    let counter = Rc::clone(&counter);
    let main = main.dyn_into::<HtmlElement>().map_err(|_| ())?;
    let loader = loader.dyn_into::<HtmlElement>().map_err(|_| ())?;

    Closure::wrap(Box::new(move || {
      let mut counter_value = counter.borrow_mut();
      if *counter_value >= 1.0 {
        let _ = main.style().set_property("opacity", "1");
        let _ = loader.style().set_property("display", "none");
      } else {
        *counter_value += 0.1;
        let _ = main
          .style()
          .set_property("opacity", &counter_value.to_string());
      }
    }) as Box<dyn FnMut()>)
  };

  window
    .set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 50)
    .map_err(|_| ())?;
  closure.forget(); // We don't want to drop the closure
  Ok(())
}
