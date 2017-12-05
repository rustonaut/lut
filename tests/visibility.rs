#[macro_use]
extern crate lut;
#[macro_use]
extern crate lazy_static;

#[test]
fn mod_visibility() {
    // if this mod compiles the test if successful
    // it's meant to check that all parts in the macros
    // use the `$crate::` prefix, and all thinks by the
    // macros are public
}

new_table! {
    flags [E11, E12, E13, E14],
    table TableToCheckMacroExpansioWithMoreThanOneOrTwoElements[u8; 4] = [
        E11, E11, E12, E14
    ]
}

new_table! {
    flags [A11, A12],
    table Tab1[u8; 4] = [
        A11, A11|A12, ,
    ]
}

new_table! {
    flags [A21],
    table Tab2[u8; 4] = [
        A21, , A21,
    ]
}

new_table! {
    flags [A31],
    table Tab3[u8; 4] = [
        A31, , , A31
    ]
}

merge_tables! {
    table Tab12[u8;4] = Tab1 [ A11, A12 ], Tab2 [A21]
}

merge_tables! {
    table Tab123[u8; 4] = Tab1 [ A11, A12 ], Tab2 [ A21 ], Tab3 [ A31 ]
}

accessor_all!{ A11AndA12 = A11 & A12 }
accessor_any!{ A11OrA21 = A11 | A21 }


