use std::{mem, ops::DerefMut, path::PathBuf, sync::Mutex};

use rust_bert::{
    marian::{MarianSourceLanguages, MarianTargetLanguages},
    pipelines::{
        common::{ModelResource, ModelType},
        translation::{Language, TranslationConfig, TranslationModel},
    },
    resources::LocalResource,
};
use tch::Device;

/// [download here](https://huggingface.co/Helsinki-NLP/opus-mt-zh-en)
const RESOURCE_PATH: &str = "/home/gxy/nlp-resource";

fn model_resource() -> String {
    format!("{}/rust_model.ot", RESOURCE_PATH)
}

fn config_resource() -> String {
    format!("{}/config.json", RESOURCE_PATH)
}

fn vocab_resource() -> String {
    format!("{}/vocab.json", RESOURCE_PATH)
}

fn spm_resource() -> String {
    format!("{}/source.spm", RESOURCE_PATH)
}

fn load_model() -> anyhow::Result<TranslationModel> {
    let model_resource = ModelResource::Torch(Box::new(LocalResource::from(PathBuf::from(
        model_resource(),
    ))));

    let config_resource = LocalResource::from(PathBuf::from(config_resource()));
    let vocab_resource = LocalResource::from(PathBuf::from(vocab_resource()));
    let spm_resource = LocalResource::from(PathBuf::from(spm_resource()));

    let source_languages = MarianSourceLanguages::CHINESE2ENGLISH;
    let target_languages = MarianTargetLanguages::CHINESE2ENGLISH;

    let translation_config = TranslationConfig::new(
        ModelType::Marian,
        model_resource,
        config_resource,
        vocab_resource,
        Some(spm_resource),
        source_languages,
        target_languages,
        Device::cuda_if_available(),
    );

    let model = TranslationModel::new(translation_config)?;

    Ok(model)
}

/// TODO
static GLOBAL_MODEL: Mutex<Option<TranslationModel>> = Mutex::new(None);

pub fn acquire_model() -> &'static TranslationModel {
    let mut global = GLOBAL_MODEL.lock().unwrap();

    if let None = global.deref_mut() {
        *global = Some(load_model().unwrap());
    }

    unsafe { mem::transmute(global.as_ref().unwrap()) }
}

pub fn translate(input: String) -> String {
    let model = acquire_model();

    let output = model.translate(&[input], None, Language::English).unwrap();

    output.join(".")
}
