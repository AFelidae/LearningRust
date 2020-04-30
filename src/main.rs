fn main() {
    let normal_array: [char; 11] = ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd', '!']; //['f'; 10];
    for y in 0..normal_array.len(){
        for x in 0..normal_array.len(){
            print!("{}",normal_array[(y+x)%normal_array.len()]);
        }
        println!("");
    }
}
