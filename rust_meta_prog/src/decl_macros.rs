mod decl_macros {
	#[macro_export]
	macro_rules! area_and_perimeter {
        ($struct_id:ty, $($x:literal),* $(,)?) => {
            impl $struct_id {
                pastey::paste! {
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

	pub(crate) use area_and_perimeter;
}
