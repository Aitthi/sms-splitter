use sms_splitter::SplitSms;

fn main(){
    let info = SplitSms::default().split("Hello World!".to_string());
    println!("{:#?}", info);
}