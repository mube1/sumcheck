
use itertools::Itertools;
mod parser;
// use std::env;

fn cat(a: &[i32],n:&[i32], b: &[i32]) -> Vec<i32> {    [a,n, b].concat() }


pub struct Proover { first_value:i32, pub polynomial: parser::Polynomial}


pub struct Verifier { pub p:Proover ,  r: Vec<i32>}
 
impl Proover{
    // fn s(&self,round:usize,x:i32, r: &[i32])->i32{
    pub fn new(expression:String)->Self{
        Proover{
            first_value:0,
            polynomial: parser::Polynomial::new(expression)
        }
    }

}

pub trait Verfication {
    fn verify(&self,round:usize)->bool;
}

impl Verfication for Verifier{
    fn verify(&self,round:usize)->bool{
        let values:[i32;2]=[self.p.s_round(round,0,self.r.clone(),self.p.polynomial.nov.into()) ,
        self.p.s_round(round,1,self.r.clone(),self.p.polynomial.nov.into())];
        
            
        if round==0{                           
                return values[0]+values[1]==self.p.first_value;
        }
        else{           
                
                return values[0]+values[1]== self.p.s_round(round-1,self.r[round-1],self.r.clone(),self.p.polynomial.nov.into());
         
        }
        
      
    }
}

pub fn sumcheck_protocol(v:Verifier,round:usize)->bool{    
    for i in 0..(round){
                
                if v.verify(i) {  continue;     }
                else{ return false;    }    
    }
    true

}

trait Srounds{
    // fn s_round(&self,round:usize,x:i32,r:Vec<i32>)->i32;
    fn s_round(&self,round:usize,x:i32,r:Vec<i32>,nov:usize)->i32;
}

impl Srounds for Proover{
    fn s_round(&self,round:usize,x:i32,random:Vec<i32>,nov:usize)->i32{

        let to_be_replaced: usize=nov-(round+1);
        let possible_values=(0..to_be_replaced).map(|_| 0..2).multi_cartesian_product();
        
        let mut result :i32=0;

        for values in possible_values{
            let vector:Vec<i32>=cat(&r,&[x],&values.to_vec());            
            result+=self.polynomial.evaluate_polynomial(vector);
        }
        result

    }
}