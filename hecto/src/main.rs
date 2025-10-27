use std::io::{self,Read};

fn main() {

    for b in io::stdin().bytes(){
        println!("{:?}",b);
        let c = b.unwrap() as char;

        println!("{}",c)
    }
    
}
