mod decl_macros;
use proc_lib::comp;
struct Square;
area_and_perimeter!(Square, 10, 21, 30);
getter_and_setter!(
	pub struct Point {
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub label: String,
	}
);

fn main() {
	let p = Point::new(23, 32, 78, "vector 1".to_string());
	println!("{}", p.get_x());
	println!("{}", Square::area_10());
	println!("{}", Square::perimeter_21());
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_point() {
		let mut p = Point::new(1, 2, 3, String::from("vector"));
		assert_eq!(*p.get_x(), 1);
		*p.get_mut_y() = 5;
		assert_eq!(*p.get_y(), 5);
		p.set_z(45);
		assert_eq!(*p.get_z(), 45);
	}

	#[test]
	fn test_square() {
		assert_eq!(Square::area_10(), 100);
		assert_eq!(Square::perimeter_21(), 84);
		assert_eq!(Square::area_30(), 900);
	}

	#[test]
	fn test_list_comp() {
		let xt = vec![1,2,3,4,5];
		let z : Vec<_> = comp!(x*2 for x in xt if x > 2).collect();
		assert_eq!(z,[6,8,10]);
	}
}
