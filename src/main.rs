#[warn(dead_code)]
 use std::env;

// mod parser;
mod sumcheck;

// use itertools::Itertools;
// use parser;


fn main() {
    // x1*x1*x1*2+x1*x3+x2*x3
    let args: Vec<String> = env::args().collect();
    let expression :&String = &args[1];
    // let first_value :i32 =;
    // let polynomial=parser::Polynomial::new(expression);    
    let mut proover = sumcheck::Proover::new(expression.to_string());
    proover.polynomial.count_variables();
    proover.polynomial.find_terms();
    proover.first_value= args[2].parse::<i32>().unwrap();
    // println!("first value read {}", proover.first_value*1);
    let prime:i32=proover.polynomial.generate_prime();
    
    let mut verifier=sumcheck::Verifier{p:proover,
        rv:vec![],
        prime:prime}; 
    println!("generated prime {}", verifier.prime);
    if sumcheck::sumcheck_protocol(&mut verifier){
        println!("Accept");
    }
    else{
        println!("Reject");
    }
    


}

