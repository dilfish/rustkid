fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v2.push(5);
    let third: &i32 = &v2[2];
    println!("The third element is {}", third);
    println!("Hello, world!{:?} , {:?}", v, v2);
   for i in &v {
    println!("{}", i);
}
   for i in &mut v2 {
    *i += 50;
}
}
