fn main(){
    let x: i32 =-50;
    let w: u64 =40;
    let quest: bool =true;
    let z: f64 = 2.10;
    let letter: char = 'm';
    let numbers_ :[i32;5]=[1,2,3,4,5];
    let fruits:[&str;3]=["apple","banana","Lemon"];
    let human:(String,i32,bool) =("ali".to_string(),42,false);
    let numbers__:&[i32] =&[1,2,3,4,5];
    let animals:&[&str] =&["tiger","lion"];
    let books:&[String] =&["book1".to_string(),"book2".to_string()];
    let names:&[String] =&["ahmed".to_string(),"ali".to_string()];
    println!("hello,world!");
    println!("x is {}",x);
    println!("w is {}",w);
    println!("this is {}",quest);
    println!("your score is {}",z);
    println!("your gender is: {}",letter);
    println!("numbers array: {:?}",numbers_);
    println!("fruit is:{:?}",fruits);
    println!("fruit is :{:?}",fruits[0]);
    println!("fruit is :{:?}",fruits[1]);
    println!("fruit is :{:?}",fruits[2]);
    println!("human : {:?}",human);
    println!("numbers: {:?}",numbers__);
    println!("animals: {:?}",animals);
    println!("books: {:?}",books);
    println!("names: {:?}",names);
    hello_world();
    tell_hight(172);
}
fn tell_hight(hight:u32){
    println!("my hight is : {} cm.",hight);
}
fn hello_world(){
    println!("hello, world!");

}