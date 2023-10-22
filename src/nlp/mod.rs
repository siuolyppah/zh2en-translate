use rust_bert::pipelines::translation::Language;

pub mod resource;

pub async fn translate(input: String) -> String {
    let model = resource::acquire_model().await;

    let output = model
        .translate(&[&input], None, Language::English)
        .expect(&format!("translation failed, the input is {}", &input));

    output.join(".")
}

#[cfg(test)]
mod tests {
    use super::translate;

    #[tokio::test]
    pub async fn translate_test() {
        let result = translate("待翻译为英文的中文字符串".into()).await;

        println!("{}", result);
    }
}
