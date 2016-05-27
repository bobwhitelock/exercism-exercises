
pub fn hello(name: Option<&str>) -> String {
     let recipient = match name {
         Some(value) => value,
         None => "World",
    };
    format!("Hello, {}!", recipient).to_string()
}
