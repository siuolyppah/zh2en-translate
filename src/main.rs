pub mod rpc;
pub mod translate;

fn main() {
    // let source_sentence = "待翻译为英文的中文字符串";
    //
    // println!("{}", translate::translate(source_sentence.into()).unwrap());
    //
    // println!(
    //     "{}",
    //     translate::translate("这是一个用于测试速度的案例".into()).unwrap()
    // );

    rpc::start_request_handle_loop(translate::translate);
}
