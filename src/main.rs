#[warn(dead_code)]
 use std::env;

// mod parser;
mod sumcheck;

// use itertools::Itertools;
// use parser;


fn main() {
    let args: Vec<String> = env::args().collect();
    let expression :&String = &args[1];
    // let polynomial=parser::Polynomial::new(expression);    
    // let mut proover = 
    let  verifier=sumcheck::Verifier{p:sumcheck::Proover::new(expression.to_string()),r:[2,3,6].to_vec()};
    // struct Verifier { p:Proover ,  r: Vec<i32>}
 

    if sumcheck::sumcheck_protocol(verifier,verifier.p.polynomial.nov.into()){
        println!("Accept");
    }
    else{
        println!("Reject");
    }
    


}

