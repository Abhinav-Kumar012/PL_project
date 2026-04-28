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
	let xt = vec![1, 2, 3, 4, 5];
	let xt2 = vec![2, 3, 4, 55];
	let p: Vec<_> =
		comp![(x,z)for x in xt if x > 2 if x < 5 for z in xt2.clone() if z < 4].collect();
	println!("{p:?}");
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
		let xt = vec![1, 2, 3, 4, 5];
		let xt2 = vec![2, 3, 4, 55];
		let p: Vec<_> =
			comp![(x,z) for x in xt if x > 2 if x < 5 for z in xt2.clone() if z < 4].collect();
		assert_eq!(p, [(3, 2), (3, 3), (4, 2), (4, 3)]);
	}
}
