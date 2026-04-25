mod decl_macros;
struct Square;
area_and_perimeter!(Square, 10, 21, 30);
getter_and_setter!(pub struct Point {
    pub x : i32,
    pub y : i32,
    pub z : i32,
    pub label : String,
});

fn main() {
    let p = Point::new(23, 32,78,"vector 1".to_string());
    println!("{}", p.get_x());
	println!("{}", Square::area_10());
	println!("{}", Square::perimeter_21());
}
