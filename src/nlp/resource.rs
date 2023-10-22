use std::{
    mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::Mutex,
};

use anyhow::anyhow;
use rust_bert::{
    marian::{MarianSourceLanguages, MarianTargetLanguages},
    pipelines::{
        common::{ModelResource, ModelType},
        translation::{TranslationConfig, TranslationModel},
    },
    resources::LocalResource,
};
use tch::Device;

use crate::config::{REMOTE_RESOURCE_URL_PREFIX, RESOURCE_PATH};
use crate::net;

macro_rules! resource_local_path {
    ($file:expr) => {
        format!("{}/{}", RESOURCE_PATH.deref(), $file)
    };
}

macro_rules! resource_remote_path {
    ($file:expr) => {
        format!("{}/{}", REMOTE_RESOURCE_URL_PREFIX, $file)
    };
}

async fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    match tokio::fs::metadata(path).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

macro_rules! check_or_download_resource {
    ($file_name:literal ) => {{
        let res_local_path = resource_local_path!($file_name);
        if !file_exists(&res_local_path).await {
            if let Err(_) =
                net::download_to(&resource_remote_path!($file_name), &res_local_path).await
            {
                return Err(anyhow!(
                    "file \"{}\" does not exist, and the download failed.",
                    res_local_path
                ));
            }
        }
        Ok(LocalResource::from(PathBuf::from(res_local_path)))
    }};
}

macro_rules! define_resource_fn {
    ($fn_name:ident, $file_name:expr ) => {
        async fn $fn_name() -> anyhow::Result<LocalResource> {
            check_or_download_resource!($file_name)
        }
    };
}

define_resource_fn!(model_resource, "rust_model.ot");
define_resource_fn!(config_resource, "config.json");
define_resource_fn!(vocab_resource, "vocab.json");
define_resource_fn!(spm_resource, "source.spm");

async fn load_model() -> anyhow::Result<TranslationModel> {
    let model_resource = ModelResource::Torch(Box::new(model_resource().await?));

    let config_resource = config_resource().await?;
    let vocab_resource = vocab_resource().await?;
    let spm_resource = spm_resource().await?;

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

pub async fn acquire_model() -> &'static TranslationModel {
    let mut global = GLOBAL_MODEL.lock().unwrap();

    if let None = global.deref_mut() {
        *global = Some(load_model().await.expect("model load fail"))
    }

    unsafe { mem::transmute(global.as_ref().unwrap()) }
}
