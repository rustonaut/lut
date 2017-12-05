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

__new_table! {
    (pub) flags [E11, E12, E13, E14],
    (pub) table TableToCheckMacroExpansioWithMoreThanOneOrTwoElements[u8; 4] = [
        E11, E11, E12, E14
    ]
}

__new_table! {
    (pub) flags [A11, A12],
    (pub) table Tab1[u8; 4] = [
        A11, A11|A12, ,
    ]
}

__new_table! {
    (pub) flags [A21],
    (pub) table Tab2[u8; 4] = [
        A21, , A21,
    ]
}

__new_table! {
    (pub) flags [A31],
    (pub) table Tab3[u8; 4] = [
        A31, , , A31
    ]
}

merge_tables! {
    table Tab12[u8;4] = Tab1 [ A11, A12 ], Tab2 [A21]
}

merge_tables! {
    table Tab123[u8; 4] = Tab1 [ A11, A12 ], Tab2 [ A21 ], Tab3 [ A31 ]
}

accessor_all!{ pub A11AndA12 = A11 & A12 }
accessor_any!{ pub A11OrA21 = A11 | A21 }


mod compile_pub_flags_priv_table {
    new_table! {
        pub flags { F1, F2 }
        struct Table {
            static data: [u8; 3] = [
                F1, F1|F2, F2
            ]
        }
    }
}
mod compile_priv_flags_pub_table {
    new_table! {
        flags { F1, F2 }
        pub struct Table {
            static data: [u8; 3] = [
                F1, F1|F2, F2
            ]
        }
    }
}
mod compile_pubcrate_both {
    new_table! {
        pub(crate) flags { F1, F2 }
        pub(crate) struct Table {
            static data: [u8; 3] = [
                F1, F1|F2, F2
            ]
        }
    }
}


