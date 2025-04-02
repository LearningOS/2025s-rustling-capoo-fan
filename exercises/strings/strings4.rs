// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");//切片类型返回str
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());//与to_string和string::from等价
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));//格式化字符串，返回String
    string_slice(&String::from("abc")[0..1]);//切片，返回str
    string_slice("  hello there ".trim());//trim()返回str，去除首尾空格
    string("Happy Monday!".to_string().replace("Mon", "Tues"));//replace()返回String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());//返回String
}
