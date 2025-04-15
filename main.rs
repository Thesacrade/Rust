use std::io ;

fn main() {    
    let mut input = String::new();
    println!("Enter your score: ");
    io::stdin().read_line(&mut input).expect("Could not read input!");
    let score: u32 =  match input.trim().parse::<u32>() {
        Ok(num)=> num,
        Err(_)=>{
            println!("Please enter a valid score!");
            return;
        } 
    };

    if score > 100 {
        println!("Enter a score from 0 to 100");
        return;
    }
    let grade = grade_score(score);
    println!("Your grade is a {} of {}", grade, score);

}
fn grade_score(score :u32) -> char{
    let grade:char;

    if score > 89 {
         grade = 'A';       
   }else if score > 79{
        grade = 'B';
   }else if  score > 69{
       grade = 'C';
   }else if score > 59{
        grade = 'D';       
   }else{
        grade = 'F';
   }
   grade
}