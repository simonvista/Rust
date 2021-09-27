// Structs - Used to create custom data types

// traditional struct
struct Color {
    red:u8,
    green:u8,
    blue:u8
}
// tuple struct
struct Color1(u8,u8,u8);
struct Person{
    first_name:String,
    last_name:String
}
impl Person {
    // constructor person
    fn new(first:&str,last:&str)->Person{
        Person{
            first_name:first.to_string(),
            last_name:last.to_string()
        }
    }
    // getter
    fn full_name(&self)->String {
        format!("{} {}",self.first_name,self.last_name)
    }   
    // setter
    fn set_last_name(&mut self, last:&str){
        self.last_name=last.to_string();
    }
    // name to tuple
    fn to_tuple(self)->(String,String) {
        (self.first_name,self.last_name)
    }
}

pub fn run() {
    // traditional struct
    let mut c=Color{
        red:255,
        green: 0,
        blue:0
    };
    c.red=200;
    println!("Color struct values: {} {} {}",c.red,c.green,c.blue);

    // tuple struct
    let mut c=Color1(255,0,0);
    c.0=200;
    println!("Color1 struct values: {:?}",(c.0,c.1,c.2));

    let mut p=Person::new("John", "Doe");
    p.first_name="Jane".to_string();
    println!("person info: {} {}",p.first_name,p.last_name);
    println!("person info: {}", p.full_name());
    p.set_last_name("Ben");
    println!("renewed name: {}",p.full_name());
    println!("person info in tuple: {:?}",p.to_tuple());
}