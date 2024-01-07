#[warn(dead_code)]
//  use std::env;

 pub struct Polynomial{
    pub terms:Vec<String>,
    pub nov:u8,
    pub expression:String
 }




 impl Polynomial{
    pub fn new(expression:String) -> Polynomial {
        
        Polynomial {
            expression: expression,
            terms:vec![],
            nov: 0,
    
      
        }
    }

    pub fn count_variables(&mut self){
        let mut start=false;       

    
        for ch in self.expression.chars(){
            if ch=='x'{
                start=true; 
                self.nov=1;
                continue;
            }
            if (ch as i32 >=48 && ch as i32 <=58) && start{
                if ch as u8 >= self.nov{self.nov=ch as u8-48;}
                else {self.nov+=1;}
                start=false;
            }
        }
        // self.prim
    }
    pub fn generate_prime(&self)->i32{
                // (2^self.nov as i32)-1 as i32
        // i32::pow(2,self.nov as u32)-1
        191
    }
    
    pub fn evaluate_term(&self,term:String,_x:Vec<i32>)->i32{
        // println!("\t evauating {} at {:?} ... ", term, _x);
        let mut result : i32= 1;
        let mut cond=false;
        let mut counter=0;
        for ch in term.chars(){
        counter+=1;
            if cond{   
                cond=false;                        
                result=result*_x[(ch as i32-48) as usize-1 ] as i32;
                continue;
            }
    
            
            if  ch as i32 >=48 && ch as i32 <=58{     
                    result=result*(ch as i32-48) as i32;        
                    }
            else{ 
    
                if ch=='/'{
                    
                    return result/self.evaluate_term(term.to_string()[counter..term.len()].to_string(),_x);
                }
                if  ch != '*' {cond=true;}
                
            }
            
        }

        result
        
    }


    pub fn find_terms(&mut self){
        // let mut terms:Vec<String>= vec![];
        println!("Finding terms ... ");
        let mut index:usize=0;
        let mut f_index:usize=0;
        for ch in self.expression.chars(){
            if ch == '+' || ch == '-' {
            self.terms.push(self.expression[f_index..index].to_string());
            if ch=='+'{self.terms.push('+'.to_string());        }
            else {self.terms.push('-'.to_string());        }

            f_index=index+1;
            }
            index+=1;
            
        }
        self.terms.push(self.expression[f_index..].to_string());
                
    }

    pub    fn evaluate_polynomial(&self,_x:Vec<i32>)->i32{
        println!("\t evauating {} at {:?} ... ", self.expression, _x);
        let mut result:i32=0;
        let mut sub= false;
        for term in self.terms.iter(){
            if term == "-"{ sub=true;continue;}
            if term == "+"{continue;}
            if sub{ result-=self.evaluate_term(term.clone(),_x.clone());  sub=false;
            }
            else{   result+=self.evaluate_term(term.clone(),_x.clone());}
    
    

        }
        result
    }
 }
