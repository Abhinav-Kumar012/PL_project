mod decl_macros;
struct Square;
area_and_perimeter!(Square, 10, 21, 30);

fn main() {
	println!("{}", Square::area_10());
	println!("{}", Square::perimeter_21());
}
