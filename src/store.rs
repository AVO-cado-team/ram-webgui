use std::collections::HashSet;

use gloo::storage::{LocalStorage, Storage};
use monaco::api::TextModel;
use ramemu::registers::Registers;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::code_editor::DEFAULT_CODE;

#[derive(Default, PartialEq, Store, Clone, Serialize, Deserialize)]
pub struct Store {
    pub breakpoints: HashSet<usize>,
    pub read_only: bool,
    pub current_debug_line: usize,
    regisers: RegistersWrapper,
    text_model: TextModelWrapper,
}

impl Store {
    pub fn get_model(&self) -> &TextModel {
        &self.text_model.0
    }
    pub fn set_model(&mut self, model: TextModel) {
        self.text_model = TextModelWrapper(model);
    }
    pub fn get_registers(&self) -> &Registers<i64> {
        &self.regisers.0
    }
    pub fn set_registers(&mut self, registers: Registers<i64>) {
        self.regisers = RegistersWrapper(registers);
    }
}

#[derive(PartialEq, Clone)]
struct TextModelWrapper(TextModel);

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
        Ok(Self(text_model))
    }
}

impl Default for TextModelWrapper {
    fn default() -> Self {
        let code: String = LocalStorage::get("code").unwrap_or_else(|_| DEFAULT_CODE.to_string());
        let text_model = TextModel::create(code.as_str(), Some("ram"), None)
            .expect("Failed to create text model");
        Self(text_model)
    }
}

#[derive(PartialEq, Clone, Default)]
struct RegistersWrapper(Registers<i64>);

impl Serialize for RegistersWrapper {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(std::collections::HashMap::<(), ()>::default())
    }
}

impl<'de> Deserialize<'de> for RegistersWrapper {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        Ok(Self(Registers::default()))
    }
}
