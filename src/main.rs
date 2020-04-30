use std::io;
use std::io::prelude::*;

extern crate rand;
use rand::Rng;

fn main() {
    //letter_count();
    //hello_world();
    //sum();
    random_wall();
}

fn random_wall(){
    for y in 0..20{
        let mut layer: String = String::from("");
        for x in 0..20{
            if rand::random() {
                layer += "1";
            } else {
                layer += "0";
            }
        }
        println!("{}", layer)
    }
}

fn sum(){
    let mut total: i32 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let some_string = line.unwrap();
        match some_string.parse::<i32>() {
            Ok(n) => {
                total += n;
                println!("Your total is {}", total);
            },
            Err(e) => println!("Enter a heccing integer >:C"),
          } 
    }
}



fn letter_count(){
    let stdin = io::stdin();
    for line in stdin.lock().lines() { //Im sure theres a nicer way of taking input
        let some_string = line.unwrap();
        println!("{}", some_string);
        println!("Is {} letters long", some_string.len());
        break;
    }
}

fn hello_world(){ //Fancy hello world function
    let normal_array: [char; 11] = ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd', '!']; //['f'; 10];
    for y in 0..normal_array.len(){
        for x in 0..normal_array.len(){
            print!("{}",normal_array[(y+x)%normal_array.len()]);
        }
        println!("");
    }
}