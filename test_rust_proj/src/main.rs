mod mod1;//mod 提供查找模块的能力
use mod1::{Cats,Tail};//use 提供缩写的能力
mod foldmod;
fn main() {
    
    let a = [0,1,2,3];
    //for in
    for num in a{
        println!("{num}");
    }
    let _b:&[u32] = &[0,1,2,3];
    file_content();
    fold_content();
}
fn file_content(){
    let mut c = Cats::new();
    c.tails.push(Tail{length:0});
    c.tails.push(Tail{length:1});
    c.show();
}
fn fold_content(){
    println!("{}",foldmod::fruit::APPLE);
    
}       


