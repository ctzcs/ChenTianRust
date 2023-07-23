use std::fs;
fn main() {
    //访问url
    let url = "https://www.rust-lang.org/";
    let output = "rust-office-web.md";
    println!("Fetching url:{}",url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    //转换成md
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output,md.as_bytes()).unwrap();
    println!("Converted md has been saved in {}", output);
}
