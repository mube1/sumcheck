
use itertools::Itertools;
mod parser;
// use rand::Rng;

fn cat(a: &[i32],n:&[i32], b: &[i32]) -> Vec<i32> {    [a,n, b].concat() }
fn _cat2(a: &[i32],b: &[i32]) -> Vec<i32> {    [a, b].concat() }


pub struct Proover { pub first_value:i32, pub polynomial: parser::Polynomial}


pub struct Verifier { pub p:Proover , pub  rv: Vec<i32>, pub prime:i32}
 

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
    fn verify(&mut self,round:usize)->bool;
}

impl Verfication for Verifier{
    fn verify(&mut self,round:usize)->bool{
        
      

        // let r :i32 = [2,3,6].to_vec()[round-1];
        // self.rv.push(r);

        // fn s_round(&self,round:usize,x:i32,rv:Vec<i32>,nov:usize)->i32{
   
        let values:[i32;2]=[
            self.p.s_round(round,0,self.rv.clone(),self.p.polynomial.nov.into()) ,
            self.p.s_round(round,1,self.rv.clone(),self.p.polynomial.nov.into())
            ];  
        println!("Recent Values{:?} at round {}",values,round )  ;
        let mut result :bool=false;
               
        if round==0{        
                   
            // let value:i32=values[0]+values[1] %self.prime ;
        //   let r :i32 = [2,3,6].to_vec()[0];
            // self.rv.push(r);

            result =  (values[0]+values[1]) % self.prime ==self.p.first_value%self.prime as i32;
        }
        else{           
            // let r:i32 = rng.gen_range(0..(self.prime));
         
            let value:i32=(values[0]+values[1] )%self.prime;
            println!("Mod value {}",value);
            let _prev_value:i32=self.p.s_round(round-1,self.rv[round-1],
                // self.rv.clone(),
                self.rv[0..round-1 as usize].to_vec(),
                self.p.polynomial.nov.into())%self.prime;
            // let _prev_value:i32=self.p.s_round(round-1,self.rv[round-1],self.rv.clone(),self.p.polynomial.nov.into())%self.prime;
            
          
            
            result  = value== _prev_value;
         
        }
        let r :i32 = [2,3,6].to_vec()[round];
        self.rv.push(r);
        result
        
      
    }
}

pub fn sumcheck_protocol(v:&mut Verifier)->bool{    
    for i in 0..(v.p.polynomial.nov as i32){                
            println!("Verifying round {}", i);
            if v.verify(i.try_into().unwrap()) { 
                println!("Done round {}", i);
                println!("******************************************");
                continue;   
            }
            else{ 
                println!("failed at round {}", i);
                return false;    }    
           
    }
    true

}

trait Srounds{
    // fn s_round(&self,round:usize,x:i32,r:Vec<i32>)->i32;
    fn s_round(&self,round:usize,x:i32,rv:Vec<i32>,nov:usize)->i32;
}

impl Srounds for Proover{
    fn s_round(&self,round:usize,x:i32,rv:Vec<i32>,nov:usize)->i32{
        
        let to_be_replaced: usize=nov-(round+1);
        // let to_be_replaced: usize=nov-(round);
        let possible_values=(0..to_be_replaced).map(|_| 0..2).multi_cartesian_product();
        println!("S_round with x = {} value and r= {:?} ar round : {} with tbg= {}",x,rv, round, to_be_replaced);

        let mut result :i32=0;
        if round+1<self.polynomial.nov as usize{
            for values in possible_values{
                let vector:Vec<i32>=cat(&rv,&[x],&values.to_vec());            
                assert!(vector.len()==self.polynomial.nov.into(), "# variables in v ={} from in r={} x= {} in possible v={} ", vector.len(),rv.len(),&[x].len(),&values.to_vec().len());
                result+=self.polynomial.evaluate_polynomial(vector);
            
            }
        }
        else{
            let vector:Vec<i32>=_cat2(&rv,&[x]);       
            assert!(vector.len()==self.polynomial.nov.into(), "# variables in v ={} from in r={} x= {} ", vector.len(),rv.len(),&[x].len());
            result = self.polynomial.evaluate_polynomial(vector);
        }
    
        
        result

    }
}
