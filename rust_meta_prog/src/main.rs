use pastey::paste;

macro_rules! multi_fxn_def {
    ($struct_id:ty, $($x:expr),* $(,)?) => {
        impl $struct_id {
            paste! {
                $(
                    pub fn [<area_ $x>]() -> isize {
                        $x * $x
                    }

                    pub fn [<perimeter_ $x>]() -> isize {
                        4 * $x
                    }
                )*
            }
        }
    };
}

struct Square;

multi_fxn_def!(Square, 10,21, 30);

fn main() {
    println!("{}", Square::area_10());
    println!("{}", Square::perimeter_21());
}