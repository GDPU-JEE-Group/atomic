use std::fmt;
struct Person{
    name: String,
    age:u32,
    sex:bool
}

impl fmt::Display for Person{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"asdas: {} {} {}",self.name,self.age,self.sex)
    }
}

fn main() {
    let person = Person { name: String::from("chaixing"),age:22,sex:true };
    println!("{}", person.to_string());



    let num:f64="12".parse().unwrap();
    println!("{}",num);
}