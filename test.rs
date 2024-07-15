fn main() {  
  let number = 123.456123123;  

 
// 使用 Display trait，保留小数点后两位
println!("Number (Display): {:.2}", number);  

// 使用 Debug trait，保留小数点后两位
println!("Number (Debug): {:.2?}", number);  

// 使用科学记数法，保留小数点后两位
println!("Number (Scientific, lower): {:.2e}", number);  
println!("Number (Scientific, upper): {:.2E}", number);
}