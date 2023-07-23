//文件映射，以文件为模块
use std::fmt::Display;
//尾巴，里面装了长度
pub struct Tail
{
   pub length:i32,
}
//给尾巴实现的打印
impl Display for Tail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.length)
    }
}

pub struct Cats{
    pub tails:Vec<Tail>
}
impl Cats {
    pub fn new()->Cats
    {
        let v:Vec<Tail> = Vec::new();
        Cats { tails: (v) }           
    }

    pub fn show(&self){
        for tail in self.tails.iter(){
            println!("{}",tail);
        }   
    }
}
