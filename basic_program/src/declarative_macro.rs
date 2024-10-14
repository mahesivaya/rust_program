macro_rules! addins {
    ($a:expr,$b:expr)=>{
        $a+$b
    }
}

fn main(){
    println!("Final value is {}", addins!(1,2));
   }



// macro_rules! double {
//     ($value:expr) => { $value * 2 }
// }

// fn main() {
//     println!("{}", double!(7));
// }