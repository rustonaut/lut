#[macro_use]
extern crate lut;
#[macro_use]
extern crate lazy_static;

#[test]
fn compile_only_tests() {
    // if this mod compiles the test if successful
    // it's meant to check that all parts in the macros
    // use the `$crate::` prefix, and all thinks by the
    // macros are public
}

new_table! {
    flags { E11=E1, E12=E2, E13=E3, E14=E4 }
    struct TableToCheckMacroExpansioWithMoreThanOneOrTwoElements {
        static data: [u8; 4] = [
            E1, E3, E2, E4
        ];
    }
}

new_table! {
    pub flags { A11=A1, A12=A2 }
    pub struct Tab1 {
        static data: [u8; 4] = [
            A1, A1|A2, -, -
        ];
    }
}

new_table! {
    flags { A21=X }
    struct Tab2 {
        static data: [u8; 4] = [
            X, -, X, -
        ];
    }
}
merge_tables! {
    struct Tab12 {
        static data: [u8;4]
            = Tab1 { A11, A12 }
            + Tab2 { A21 };
    }
}


accessor_all!{ A11AndA12 = A11 & A12 }
accessor_any!{ A11OrA21 = A11 | A21 }



mod compile_pub_flags_priv_table {
    new_table! {
        pub flags { F1=F1, F2=F2 }
        struct Table {
            static data: [u8; 3] = [
                F1, F1|F2, F2
            ];
        }
    }
}
mod compile_priv_flags_pub_table {
    new_table! {
        flags { F1=F1, F2=F2 }
        pub struct Table {
            static data: [u8; 3] = [
                F1, F1|F2, F2
            ];
        }
    }
}
mod compile_pubcrate_both {
    new_table! {
        pub(crate) flags { F1=F1, F2=F2 }
        pub(crate) struct Table {
            static data: [u8; 3] = [
                F1, F1|F2, F2
            ];
        }
    }
}

mod compile_relative_path_merge {
    use super::{A11, A12};
    use super::compile_pubcrate_both::{F1, F2};

    merge_tables! {
        struct Table {
            static data: [u8; 4]
                = super::Tab1 { A11, A12 }
                + super::compile_pubcrate_both::Table { F1, F2 };
        }
    }

    merge_tables! {
        pub struct TableX {
            static data: [u8; 4]
                = super::Tab1 { A11, A12 };

        }
    }

    merge_tables! {
        pub(crate) struct TableQ {
            static data: [u8; 4]
                = super::Tab1 { A11, A12 };
        }
    }
}


mod with_attributes {

    new_table! {
        //a doc-comment is a attribute, too ;=)
        flags {
            /// document flag
            Index0=X,
            /// doc that
            Index1=Y
        }
        /// documented
        struct Table {
            static data: [u8; 2] = [ X, Y ];
        }
    }

    merge_tables! {
        /// da new table
        struct Table2 {
            static data: [u8; 2] = Table { Index0 };
        }
    }

}

mod handling_of_unexpected_cases {

//  This should not compile (and if not broken at some
//  point doesn't)
//
//  new_table! {
//      flags { F1, F2, F3, F4, F5, F6, F7, F8, F9 }
//      /// this has more flags than it can handle
//      struct Table0 {
//          static data: [u8; 1] = [ F9 ];
//      }
//  }

    new_table! {
        flags {}
        /// a table without flags
        struct Table1 {
            static data: [u8; 2] = [-,-];
        }
    }

    //[BAD] DOES NOT COMPILE, error: ambiguity: multiple successful parses
    // but it's ok, empty tables are kinda pointless
//    new_table! {
//        flags {}
//        /// a zero-sized table
//        struct Table2 {
//            static data: [u8; 0] = [];
//        }
//    }

    mod unused_flag {
        #![allow(dead_code)]
        
        new_table! {
            flags { F1=F1 }
            /// A table with a single zero cell
            struct Table3 {
                static data: [u8; 1] = [ - ];
            }
        }
    }


}