mod add1;
use add1::{Complex as Cplx1};
fn main() {
    let c1 = Cplx1::new(1.0,1f64);
    let c2 = Cplx1::new(2 as f64,2.0);
    //addTrait对于实现了Copytrait的类型，比如u32，f64等，很好用，但是
    println!("{:?}",c1 + c2);
}
