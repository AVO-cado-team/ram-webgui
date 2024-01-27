use js_sys::Object;
use monaco::sys::editor;
use yewdux::Listener;

use crate::store::Store as EditorStore;

struct EditorStoreListener {
    old: std::rc::Rc<EditorStore>,
}
impl Listener for EditorStoreListener {
    type Store = EditorStore;

    fn on_change(&mut self, _: &yewdux::Context, state: std::rc::Rc<Self::Store>) {
        let new = state.clone();
        self.old = new;
    }
}
