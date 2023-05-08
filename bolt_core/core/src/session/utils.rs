pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}
