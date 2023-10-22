use once_cell::sync::Lazy;

/// [download here](https://huggingface.co/Helsinki-NLP/opus-mt-zh-en)
pub const RESOURCE_PATH: Lazy<String> = Lazy::new(|| match home::home_dir() {
    Some(mut home) => {
        home.push("nlp-resource");
        home.to_str().unwrap().to_owned()
    }
    None => unreachable!(),
});

pub const REMOTE_RESOURCE_URL_PREFIX: &str =
    "https://huggingface.co/Helsinki-NLP/opus-mt-zh-en/raw/main";
