use std::ops::Add;
#[derive(Debug)]
pub struct Complex{
    real:f64,
    imagine:f64,
}
impl Complex{
    pub fn new(real:f64,imagine:f64)->Self{
        Self{real,imagine}
    }
}

//对Complex类型的实现
impl Add for Complex{
    type Output = Self;
    //add 的第一个参数是self，会移动所有权
    fn add(self,rhs:Self)->Self::Output{
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real,imagine)
    }
}