struct Rect {
    width : u32 ,
    height : u32
}

struct Square {
    side : u32
}


trait  Shape  {
    fn area(&self) -> u32 ;
    fn Shape_type(&self) -> String ;
}

impl Shape for  Rect {
    fn area(&self) -> u32 {
        return self.width * self.height ;
    }
    fn Shape_type(&self ) -> String {
        return "Rectangle".to_string() ;
    }
}

impl Shape for  Square {
    fn area(&self) -> u32 {
    return self.side * self.side ;
    }
    fn Shape_type(&self) -> String {
        return "Square".to_string() ;
    }
}



fn main() {
    println!("Hello, world!");

    let r : Rect = Rect { width : 10 , height : 20 };
    println!("Area : {} {}" , r.area()  , r.Shape_type());
    let s : Square = Square { side : 10 };
    println!("Area : {} {}" , s.area() , s.Shape_type());
}
