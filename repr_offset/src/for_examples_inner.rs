use crate::{Aligned, FieldOffset, Unaligned};

macro_rules! declare_example_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident;
        alignment =  $alignment:ty,
    ) => {
        $(#[$meta])*
        #[derive(Debug,Default,PartialEq,Eq)]
        pub struct $name<A,B,C,D>{
            pub a:A,
            pub b:B,
            pub c:C,
            pub d:D,
        }

        unsafe_struct_field_offsets!{
            alignment =  $alignment,
            impl[A,B,C,D] $name<A,B,C,D>{
                /// The offset of the `a` field
                pub const OFFSET_A: A;
                /// The offset of the `b` field
                pub const OFFSET_B: B;
                /// The offset of the `c` field
                pub const OFFSET_C: C;
                /// The offset of the `d` field
                pub const OFFSET_D: D;
            }
        }
    };
}

declare_example_struct! {
    /// An example `#[repr(C)]` type
    #[repr(C)]
    struct ReprC;
    alignment = Aligned,
}

declare_example_struct! {
    /// An example `#[repr(C, align(4))]` type
    #[repr(C, align(4))]
    struct ReprAlign4;
    alignment = Aligned,
}

declare_example_struct! {
    /// An example `#[repr(C, packed)]` type
    #[repr(C, packed)]
    struct ReprPacked;
    alignment = Unaligned,
}
