fn main() {
    // 在rust中没有深拷贝，但是可以使用克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    let mut x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    x = 2;
    println!("x = {}, y = {}", x, y);
    // let len = calculate_length(s1);
    // let mut s11 = String::from("hello");
    let a = no_dangle();
    println!("a = {}", a);
    // calculate_length(&mut s11);
    // println!("s11 = {}", s11);
    // println!("len = {:?},", len);
    splice();
    enum_api();
}
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
fn splice(){
  let s = String::from("hello world");

  let hello1 = &s[0..5];
  let hello2 = &s[..5];
  let world1 = &s[6..11];
  let world2 = &s[6..];

  println!("world1 = {}, world2 = {}", world1, world2);
  println!("hello1 = {}, hello2 = {}", hello1, hello2);
}
fn enum_api(){
  #[derive(Debug)]
  enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
  }
  let four = IpAddr::V4(127,0,0,1);
  let six = IpAddr::V6(String::from("::1"));
  println!("four = {:?}, six = {:?}", four, six);

  #[derive(Debug)]
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }

  impl Message {
      fn call(&self) {
          println!("{:?}",self)
      }
  }

  let m = Message::Write(String::from("hello"));
  m.call();
}
// fn calculate_length( s:&mut String) {
//   // s.len()
//   s ="123";
//   // println!("s = {}", s);
// }
