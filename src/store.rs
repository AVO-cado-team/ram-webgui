use std::collections::HashSet;

use monaco::api::TextModel;
#[cfg(not(feature = "ssr"))]
use monaco::yew::CodeEditorLink;
use ramemu::registers::Registers;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::{code_editor::DEFAULT_CODE, io::output::OutputComponentErrors};

#[cfg(feature = "ssr")]
pub fn dispatch() -> Dispatch<Store> {
    use yewdux::Context;
    thread_local! {
        static CONTEXT: Context = Default::default();
    }

    Dispatch::new(&CONTEXT.with(|context| context.clone()))
}

#[cfg(not(feature = "ssr"))]
pub fn dispatch() -> Dispatch<Store> {
    Dispatch::global()
}

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct Store {
    #[serde(skip)]
    #[cfg(not(feature = "ssr"))]
    pub editor: CodeEditorLink,
    #[serde(skip)]
    pub errors: Vec<OutputComponentErrors>,
    #[serde(skip)]
    pub read_only: bool,
    #[serde(skip)]
    pub current_debug_line: usize,
    #[serde(skip)]
    registers: Registers<i64>,
    #[serde(skip)]
    pub copy_button_state: Option<bool>,

    #[cfg(not(feature = "ssr"))]
    text_model: TextModelWrapper,
    pub breakpoints: HashSet<usize>,
    pub stdin: String,
}

impl Store {
    #[cfg(not(feature = "ssr"))]
    pub fn get_model(&self) -> &TextModel {
        &self.text_model.0
    }
    #[cfg(not(feature = "ssr"))]
    pub fn change_model(&mut self) {
        self.text_model.1 = self.text_model.1.wrapping_add(1);
    }
    pub fn get_registers(&self) -> &Registers<i64> {
        &self.registers
    }
    pub fn set_registers(&mut self, registers: Registers<i64>) {
        self.registers = registers;
    }
}

#[derive(Clone)]
struct TextModelWrapper(TextModel, u64);

impl PartialEq for TextModelWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Serialize for TextModelWrapper {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let code = self.0.get_value();
        serializer.serialize_str(code.as_str())
    }
}

impl<'de> Deserialize<'de> for TextModelWrapper {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = String::deserialize(deserializer)?;
        let text_model = TextModel::create(code.as_str(), Some("ram"), None);
        let text_model = text_model.map_err(|js_err| {
            gloo::console::error!("Failded to create text model: ", js_err);
            serde::de::Error::custom("Failed to deserialize text model")
        })?;
        Ok(Self(text_model, 0))
    }
}

impl Default for TextModelWrapper {
    fn default() -> Self {
        let text_model = TextModel::create(DEFAULT_CODE, Some("ram"), None)
            .expect("Failed to create text model");
        Self(text_model, 0)
    }
}
