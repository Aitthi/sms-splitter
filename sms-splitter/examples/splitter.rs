use sms_splitter::SplitSms;

fn main() {
    let spliter = SplitSms::default();
    let info = spliter.split("Hello World!".to_string());
    println!("{:#?}", info);
    // Unicode
    let info = spliter.split("Hello Again World! 📡📡📡 1111111111111111111111111111111111111111|222222222222222222222222222222222222222222222222222222222222222222|33333333".to_string());
    println!("{:#?}", info);
}
