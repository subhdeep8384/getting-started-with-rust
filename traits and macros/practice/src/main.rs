struct Rect {
    height : i32 ,
    width : i32 
}
trait  Shape {
    fn area (&self) -> i32  ;
    fn parameter(&self) -> i32 ;
    fn name () -> String ;
}
impl Shape for  Rect  {
    fn area(&self) -> i32 {
        return self.height * self.width;
    }
    fn parameter(&self) -> i32 {
        return self.height + self.width ;
    }
    fn name() -> String {
        return  String::from("Reactangle");
    }
}
fn get_area_parameter(s :  impl Shape) -> (i32 , i32 ){
    return  (s.area() , s.parameter());
}
    
fn main() {
    println!("Hello, world!");
    let r : Rect = Rect{
        width  :100 ,
        height : 100
    } ;
    println!("area : {} shape is {} , {:?} " , r.area() , Rect::name() , get_area_parameter(r) );
}
