use std::collections::HashSet;

use monaco::api::TextModel;
use ramemu::registers::Registers;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::code_editor::DEFAULT_CODE;

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct Store {
    pub breakpoints: HashSet<usize>,
    pub read_only: bool,
    pub current_debug_line: usize,
    pub stdin: String,
    #[serde(skip)]
    registers: Registers<i64>,
    text_model: TextModelWrapper,
}

impl Store {
    pub fn get_model(&self) -> &TextModel {
        &self.text_model.0
    }
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
