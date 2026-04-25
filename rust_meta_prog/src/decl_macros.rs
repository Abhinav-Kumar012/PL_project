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

    #[macro_export]
    macro_rules! getter_and_setter {
        ($struct_vis : vis struct $struct_id : ident {$($field_vis : vis $field : ident : $type : ty),* $(,)?}) => {
            $struct_vis struct $struct_id {
                $(
                    $field_vis $field : $type
                ),*
            }
            impl $struct_id {
                pub fn new($($field : $type),*) -> Self{
                    return Self {
                        $(
                            $field
                        ),*
                    }
                }
                pastey::paste!{
                    $(
                        pub fn [<get_ $field>](&self) -> &$type {
                            return &(self.$field)
                        }
                        pub fn [<get_mut_ $field>](&mut self) -> &mut $type {
                            return &mut (self.$field)
                        }
                        pub fn [<set_ $field>](&mut self, new_val : $type) {
                            self.$field = new_val;
                        }
                    )*
                }
            }
        };
    }

	pub(crate) use area_and_perimeter;
    pub(crate) use getter_and_setter;

}
