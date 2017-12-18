
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::cmp::Eq;
use std::marker::PhantomData;


pub trait TableValue:
Sized
+ Copy
+ BitAnd<Self, Output=Self>
+ BitAndAssign<Self>
+ BitOr<Self, Output=Self>
+ BitOrAssign<Self>
+ Eq
{
    const MAX_FLAG_COUNT: usize;
    const ZERO: Self;
}

macro_rules! impl_tv {
    ($($uxx:ident($count:tt)),*) => ($(
        impl TableValue for $uxx {
            const MAX_FLAG_COUNT: usize = $count;
            const ZERO: $uxx = 0;
        }
    )*)
}

impl_tv! {
    u8(8), u16(16), u32(32), u64(64)
}

pub trait ConstFlagCount {
    const FLAG_COUNT: usize;
}

impl ConstFlagCount for () {
    const FLAG_COUNT: usize = 0;
}

pub trait Table: Sized + ConstFlagCount {
    type Value: TableValue;

    #[inline]
    fn mask<A: Flag<Self>>(_hint: A) -> Self::Value {
        <A as Flag<Self>>::BIT_MASK
    }

    fn len() -> usize;

    fn lookup(idx: usize) -> Self::Value;

    fn check_at<A: Access<Self>>(idx: usize, accessor: A) -> bool;

    #[inline(always)]
    fn unbound_check_at<A: Access<Self>>(idx: usize, accessor: A) -> bool {
        if idx > Self::len() {
            false
        } else {
            Self::check_at(idx, accessor)
        }
    }

    #[doc(hidden)]
    fn check_flag_at<A: Flag<Self>>(idx: usize) -> bool;
}

pub trait Flag<T: Table>: Access<T> {
    const BIT_MASK: T::Value;
}

pub trait Access<T: Table>: Sized {
    fn check(&self, value: T::Value) -> bool;
}

#[doc(hidden)]
pub struct FCSum<Head: ConstFlagCount, Tail: ConstFlagCount> {
    mark: PhantomData<(Head,Tail)>
}

impl<H,T> ConstFlagCount for FCSum<H, T>
    where H: ConstFlagCount, T: ConstFlagCount
{
    const FLAG_COUNT: usize = H::FLAG_COUNT + T::FLAG_COUNT;
}


#[derive(Copy, Clone, Debug)]
pub struct EmptyFlag;

impl<T> Access<T> for EmptyFlag
    where T: Table
{
    #[inline(always)]
    fn check(&self, _value: T::Value) -> bool {
        false
    }
}


#[derive(Copy, Clone, Debug)]
pub struct NoFlagsSet;
impl<T> Access<T> for NoFlagsSet
    where T: Table
{
    #[inline(always)]
    fn check(&self, value: T::Value) -> bool {
        value == T::Value::ZERO
    }
}

///
/// # Example
//```
//new_table! {
//    flags { F1, F2, F3 }
//    struct Table1 {
//        static data: [u8;4] = [
//            F1,   ,  F2|F3,      ,
//            F2, F4,       ,    F5,
//        ]
//    }
//}
//```
///
#[macro_export]
macro_rules! new_table {
    (
        pub flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        pub struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            (pub) flags [$( $(#[$fattr])* $fname = $short_name ),*],
            (pub) table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

    (
        pub flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        pub($($vis:tt)+) struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            (pub) flags [$( $(#[$fattr])* $fname = $short_name ),*],
            (pub($($vis)+)) table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

    (
        pub flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            (pub) flags [$( $(#[$fattr])* $fname = $short_name),*],
            () table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

    //----

    (
        pub($($vis:tt)+) flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        pub struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            (pub($($vis)+)) flags [$( $(#[$fattr])* $fname = $short_name),*],
            (pub) table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

    (
        pub($($fvis:tt)+) flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        pub($($tvis:tt)+) struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            (pub($($fvis)+)) flags [$( $(#[$fattr])* $fname = $short_name),*],
            (pub($($tvis)+)) table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

    (
        pub($($vis:tt)+) flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            (pub($($vis)+)) flags [$( $(#[$fattr])* $fname = $short_name),*],
            () table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

    //-------
    (
        flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        pub struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            () flags [$( $(#[$fattr])* $fname = $short_name),*],
            (pub) table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );
    (
        flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        pub($($vis:tt)+) struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            () flags [$( $(#[$fattr])* $fname = $short_name),*],
            (pub($($vis)+)) table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );
    (
        flags {$( $(#[$fattr:meta])* $fname:ident = $short_name:ident),*}
        $(#[$attr:meta])*
        struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt] = [$($($v:tt)|*),*];
        }
    ) => (
        __new_table! {
            () flags [$( $(#[$fattr])* $fname = $short_name),*],
            () table $(#[$attr])* $name [$tp;$size] = [$($($v)|*),*]
        }
    );

}

#[doc(hidden)]
#[macro_export]
macro_rules! __new_table {
    (
        ($($flag_vis:tt)*) flags [$($(#[$fattr:meta])* $fname:ident = $short_name:ident),*],
        ($($table_vis:tt)*) table $(#[$attr:meta])* $name:ident [$tp:ty;$size:tt] = [$($($v:tt)|*),*]
    ) => (

        __new_table!{@DEF_FLAGS ($($flag_vis)*) $name [$($(#[$fattr])* $fname),*] [ ]}


        #[derive(Copy, Clone, Debug)]
        $(#[$attr])*
        $($table_vis)* struct $name;

        impl $crate::ConstFlagCount for $name {
            const FLAG_COUNT: usize = __new_table!{@COUNT [$($fname),*] []};
        }

        impl $crate::Table for $name {
            type Value = $tp;

            #[inline(always)]
            fn len() -> usize {
                $size
            }

            #[inline(always)]
            fn lookup(idx: usize) -> Self::Value {
                $(type $short_name = self::$fname;)*
                type S = $name;
                type T = $tp;
                static TABLE: [$tp;$size] = [$(
                    0$(|<__new_table!{@MAP $v} as $crate::Flag<S>>::BIT_MASK as T)*
                ),*];
                TABLE[idx]
            }

            #[inline(always)]
            fn check_at<A: $crate::Access<Self>>(idx: usize, accessor: A) -> bool {
                accessor.check(Self::lookup(idx))
            }

            #[inline(always)]
            fn check_flag_at<A: $crate::Flag<Self>>(idx: usize) -> bool {
                Self::lookup(idx) & A::BIT_MASK != <Self::Value as $crate::TableValue>::ZERO
            }
        }

        // workaround for something which looks a lot like a compiler bug
        // and prevents me from having a wild card implementation for any
        // table
        impl $crate::Flag<$name> for $crate::EmptyFlag {
            const BIT_MASK: $tp = <<$name as $crate::Table>::Value as $crate::TableValue>::ZERO;
        }
    );
    (@DEF_FLAGS ($($flag_vis:tt)*) $t:ident [] [$($inc:tt)*]) => ();
    (@DEF_FLAGS ($($flag_vis:tt)*) $table:ident
        [ $(#[$hattr:meta])* $head:ident $(, $(#[$tattr:meta])* $tail:ident)*]
        [$($inc:tt)*]
    ) => (

        #[derive(Copy, Clone, Debug)]
        $(#[$hattr])*
        $($flag_vis)* struct $head;


        impl $crate::Flag<$table> for $head {
            const BIT_MASK: <$table as $crate::Table>::Value =
                (1 << (0 $(+ $inc)*)); // as <$table as $crate::Table>::Value;
        }

        impl $crate::Access<$table> for $head {
            fn check(&self, value: <$table as $crate::Table>::Value) -> bool {
                value & <Self as $crate::Flag<$table>>::BIT_MASK !=
                <<$table as $crate::Table>::Value as $crate::TableValue>::ZERO
            }
        }

        impl<T> Into<$crate::All<T>> for $head
            where T: $crate::Table, $head: $crate::Flag<T>
        {
            #[inline(always)]
            fn into(self) -> $crate::All<T> {
                $crate::All::from_mask(<$head as $crate::Flag<T>>::BIT_MASK)
            }
        }

        impl<T> Into<$crate::Any<T>> for $head
            where T: $crate::Table, $head: $crate::Flag<T>
        {
            #[inline(always)]
            fn into(self) -> $crate::Any<T> {
                $crate::Any::from_mask(<$head as $crate::Flag<T>>::BIT_MASK)
            }
        }

        impl ::std::default::Default for $head {
            fn default() -> Self {
                $head
            }
        }

        __new_table!{@DEF_FLAGS ($($flag_vis)*) $table [$($tail),*] [$($inc)* 1]}
    );
    (@COUNT [] [$($inc:tt)*]) => (
        $($inc +)* 0;
    );
    (@COUNT [$head:ident $(, $tail:ident)*] [$($inc:tt)*]) => (
        __new_table!{@COUNT [$($tail),*] [$($inc)* 1]}
    );
    (@MAP -) => ($crate::EmptyFlag);
    (@MAP $v:tt) => ($v);
}

/// # Example
///
// ```
// merge_tables! {
//     struct Table12 {
//         static data: [u8; 4]
//             = Tab1 { A11, A12 }
//             + Tab2 { A21 }
//             + Tab3 { Ra, Ro, Ri }
//     }
// }
// ```
///
#[macro_export]
macro_rules! merge_tables {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt]
            = $first:ty { $($flag:ty),* }
            $(
                + $next:ty { $($nflag:ty),* }
            )*;
        }
    ) => (
        __merge_tables! {
            table $(#[$attr])* (pub) $name [$tp;$size] = $first [ $($flag),* ] $(, $next [$($nflag),*])*
        }
    );
    (
        $(#[$attr:meta])*
        pub($($vis:tt)+) struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt]
            = $first:ty { $($flag:ty),* }
            $(
                + $next:ty { $($nflag:ty),* }
            )*;
        }
    ) => (
        __merge_tables! {
            table $(#[$attr])* (pub($($vis)+)) $name [$tp;$size] = $first [ $($flag),* ] $(, $next [$($nflag),*])*
        }
    );
    (
        $(#[$attr:meta])*
        struct $name:ident {
            static $_f:ident: [$tp:ty;$size:tt]
            = $first:ty { $($flag:ty),* }
            $(
                + $next:ty { $($nflag:ty),* }
            )*;
        }
    ) => (
        __merge_tables! {
            table $(#[$attr])* () $name [$tp;$size] = $first [ $($flag),* ] $(, $next [$($nflag),*])*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __merge_tables {
    (table $(#[$attr:meta])* ($($vis:tt)*) $name:ident [$tp:ty;$size:tt] = $($ct:ty [$($cf:ty),*]),*) => (

        #[derive(Copy, Clone)]
        $(#[$attr])*
        $($vis)* struct $name;

        impl $crate::ConstFlagCount for $name {
            const FLAG_COUNT: usize = 0 $(+ <$ct as $crate::ConstFlagCount>::FLAG_COUNT)*;
        }

        impl $crate::Table for $name {
            type Value = $tp;

            #[inline(always)]
            fn len() -> usize {
                $size
            }

            #[inline(always)]
            fn lookup(idx: usize) -> Self::Value {
                lazy_static! {
                    static ref TABLE: [$tp;$size] = {
                        let mut res = [<$tp as Default>::default();$size];
                        for (idx, field) in res.iter_mut().enumerate() {
                            __merge_tables!{ @MERGE_ITER_STEP
                                $tp, idx, field, $name,
                                $crate::FCSum<(), ()>,
                                [$($ct [$($cf)*])*]
                            }
                        }
                        res
                    };
                }
                TABLE[idx]
            }

            #[inline(always)]
            fn check_at<A: $crate::Access<Self>>(idx: usize, accessor: A) -> bool {
                accessor.check(Self::lookup(idx))
            }

            #[inline(always)]
            fn check_flag_at<A: $crate::Flag<Self>>(idx: usize) -> bool {
                Self::lookup(idx) & A::BIT_MASK != <Self::Value as $crate::TableValue>::ZERO
            }
        }


        __merge_tables! {@MERGE_FLAG_IMPL
            $name, $tp, $name,
            $crate::FCSum<(),()>,
            [ $($ct [$($cf)*])* ] }
    );
    ( @MERGE_FLAG_IMPL $new_table:ident, $tp:ty, $fc_total:ident, $fc_prev:ty, []) => ();
    ( @MERGE_FLAG_IMPL $new_table:ident, $tp:ty, $fc_total:ident,
        $fc_prev:ty,
        [ $current_table:ty [ $($current_flag:ty)* ] $($tail_t:ty [$($tail_f:ty)*])* ]
    ) => (
        //  $fc_prev = FCSum<$current, $prev_count>
        //           = FCSum<Table1, FCSum<Table2,
        //              ...FCSum<TableX, FCSum<(),()>>...>>
        $(
            impl $crate::Flag<$new_table> for $current_flag {
                const BIT_MASK: $tp =
                    //FIXME we need a `as $tp` to go from u8->u16 etc. but sadly it
                    // also allows the other way around u16-u8
                    // we might be able to do some tricks with having a nop bit shift (which is optimized
                    // out) which is larger, as a >8 for u8, ... bit shift _does fail at compiler time_
                    // just when we shift less but our mask is bigger it hits
                    (<$current_flag as $crate::Flag<$current_table>>::BIT_MASK as $tp)
                    << (<$fc_total as $crate::ConstFlagCount>::FLAG_COUNT
                        - <$crate::FCSum<$current_table, $fc_prev> as $crate::ConstFlagCount>::FLAG_COUNT);
            }
            impl $crate::Access<$new_table> for $current_flag {
                #[inline(always)]
                fn check(&self, value: <$new_table as $crate::Table>::Value) -> bool {
                    value & <Self as $crate::Flag<$new_table>>::BIT_MASK !=
                    <<$new_table as $crate::Table>::Value as $crate::TableValue>::ZERO
                }
            }
        )*
        __merge_tables!{ @MERGE_FLAG_IMPL
            $new_table, $tp, $fc_total,
            $crate::FCSum<$current_table, $fc_prev>,
            [ $($tail_t [ $($tail_f)* ])* ] }
    );
    ( @MERGE_ITER_STEP $tp:ty, $idx:ident, $field:ident, $total_fc:ident, $fc_prev:ty, []) => ();
    ( @MERGE_ITER_STEP $tp:ty, $idx:ident, $field:ident, $total_fc:ident,
        $fc_prev:ty,
        [ $current_table:ty [ $($current_flag:ty)* ] $($tail_t:ty [$($tail_f:ty)*])* ]
    ) => (
        {$(
            if <$current_table as $crate::Table>::check_flag_at::<$current_flag>($idx) {
                *$field = *$field | (
                    <$current_flag as $crate::Flag<$current_table>>::BIT_MASK
                    << (<$total_fc as $crate::ConstFlagCount>::FLAG_COUNT
                        - <$crate::FCSum<$current_table, $fc_prev> as $crate::ConstFlagCount>::FLAG_COUNT)
                ) as $tp;
            }
        )*}
        __merge_tables! { @MERGE_ITER_STEP
            $tp, $idx, $field, $total_fc,
            $crate::FCSum<$current_table, $fc_prev>,
            [$($tail_t [$($tail_f)*])*]
        }
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Any<T: Table> {
    mask: T::Value
}

impl<T> Any<T>
    where T: Table
{
    #[inline(always)]
    pub fn mask(self) -> T::Value {
        self.mask
    }

    #[inline(always)]
    pub fn new<F: Flag<T>>(_ty_flag: F) -> Self {
        Any { mask: <F as Flag<T>>::BIT_MASK }
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Any { mask: T::Value::ZERO }
    }

    #[inline(always)]
    pub fn from_mask(mask: T::Value) -> Self {
        Any { mask }
    }
}


impl<T, I: Into<Any<T>>> BitOr<I> for Any<T>
    where T: Table
{
    type Output = Self;

    #[inline(always)]
    fn bitor(self, other: I) -> Self {
        Any { mask: self.mask | other.into().mask }
    }
}

impl<T, I: Into<Any<T>>> BitOrAssign<I> for Any<T>
    where T: Table
{

    #[inline(always)]
    fn bitor_assign(&mut self, other: I) {
        self.mask |=  other.into().mask
    }
}

impl<T> Access<T> for Any<T>
    where T: Table
{
    #[inline(always)]
    fn check(&self, value: T::Value) -> bool {
        value & self.mask != T::Value::ZERO
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct All<T: Table> {
    mask: T::Value
}

impl<T> All<T>
    where T: Table
{
    #[inline(always)]
    pub fn mask(self) -> T::Value {
        self.mask
    }

    #[inline(always)]
    pub fn new<F: Flag<T>>(_ty_flag: F) -> Self {
        All { mask: <F as Flag<T>>::BIT_MASK }
    }

    #[inline(always)]
    pub fn empty() -> Self {
        All { mask: T::Value::ZERO }
    }

    #[inline(always)]
    pub fn from_mask(mask: T::Value) -> Self {
        All { mask }
    }
}

impl<T, I: Into<All<T>>> BitAnd<I> for All<T>
    where T: Table
{
    type Output = Self;

    #[inline(always)]
    fn bitand(self, other: I) -> Self {
        All { mask: self.mask | other.into().mask }
    }
}

impl<T, I: Into<All<T>>> BitAndAssign<I> for All<T>
    where T: Table
{

    #[inline(always)]
    fn bitand_assign(&mut self, other: I) {
        self.mask |=  other.into().mask
    }
}

impl<T> Access<T> for All<T>
    where T: Table
{
    #[inline(always)]
    fn check(&self, value: T::Value) -> bool {
        self.mask & value == self.mask
    }
}


#[macro_export]
macro_rules! accessor_all {
    (pub $name:ident = $($subname:ident)&+) => (
        accessor_all!{ @IMPL
            (pub) $name = $($subname)&+
        }
    );
    (pub($($vis:tt)*) $name:ident = $($subname:ident)&+) => (
        accessor_all!{ @IMPL
            (pub($($vis)*)) $name = $($subname)&+
        }
    );
    ($name:ident = $($subname:ident)&+) => (
        accessor_all!{ @IMPL
            () $name = $($subname)&+
        }
    );
    (@IMPL ($($vis:tt)*) $name:ident = $($subname:ident)&+) => (
        #[derive(Copy, Clone, Debug)]
        $($vis)* struct $name;
        impl<T: $crate::Table> $crate::Access<T> for $name
                  where $($subname: $crate::Flag<T>),*
        {
            #[inline]
            fn check(&self, value: T::Value) -> bool {
                let mask: T::Value = $(<$subname as $crate::Flag<T>>::BIT_MASK)|*;
                value & mask == mask
            }
        }
        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name
            }
        }
    )
}

#[macro_export]
macro_rules! accessor_any {
    (pub $name:ident = $($subname:ident)|+) => (
        accessor_any!{ @IMPL
            (pub) $name = $($subname)|+
        }
    );
    (pub($($vis:tt)*) $name:ident = $($subname:ident)|+) => (
        accessor_any!{ @IMPL
            (pub($($vis)*)) $name = $($subname)|+
        }
    );
    ($name:ident = $($subname:ident)|+) => (
        accessor_any!{ @IMPL
            () $name = $($subname)|+
        }
    );
    (@IMPL ($($vis:tt)*) $name:ident = $($subname:ident)|+) => (
        #[derive(Copy, Clone, Debug)]
        $($vis)* struct $name;
        impl<T: $crate::Table> $crate::Access<T> for $name
                  where $($subname: $crate::Flag<T>),*
        {
            #[inline(always)]
            fn check(&self, value: T::Value) -> bool {
                let mask: T::Value = $(<$subname as $crate::Flag<T>>::BIT_MASK)|*;
                value & mask != <T::Value as $crate::TableValue>::ZERO
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name
            }
        }
    )
}



#[cfg(test)]
mod test {
    use super::*;

    new_table! {
        flags { E11=E11, E12=E12, E13=E13, E14=E14 }
        struct TableToCheckMacroExpansioWithMoreThanOneOrTwoElements {
            static data: [u8; 4] = [
                E11, E12, E13, E14
            ];
        }
    }

    new_table! {
        flags { A11=A1, A12=A2 }
        struct Tab1 {
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

    new_table! {
        flags { A31=X }
        struct Tab3 {
            static data: [u8; 4] = [
                X, -, -, X
            ];
        }
    }


    merge_tables! {
        struct Tab12 {
            static data: [u8; 4]
                = Tab1 { A11, A12 }
                + Tab2 { A21 };
        }
    }

    merge_tables! {
        struct Tab123 {
            static data: [u8; 4]
                = Tab1 { A11, A12 }
                + Tab2 { A21 }
                + Tab3 { A31 };
        }
    }

    accessor_all!{ A11AndA12 = A11 & A12 }
    accessor_any!{ A11OrA21 = A11 | A21 }

    #[test]
    fn all_on_tab1() {
        assert!(!Tab1::check_at(0, A11AndA12));
        assert!(Tab1::check_at(1, A11AndA12));
        assert!(!Tab1::check_at(2, A11AndA12));
        assert!(!Tab1::check_at(3, A11AndA12));
    }

    #[test]
    fn any_on_tab1() {
        assert!(Tab12::check_at(0, A11OrA21));
        assert!(Tab12::check_at(1, A11OrA21));
        assert!(Tab12::check_at(2, A11OrA21));
        assert!(!Tab12::check_at(3, A11OrA21));
    }




    #[test]
    fn merge_tab1_with_tab2() {
        assert_eq!(Tab12::len(), 4);

        assert_eq!(Tab12::mask(A11), 1 << 0 + 1);
        assert_eq!(Tab12::mask(A12), 1 << 1 + 1);
        assert_eq!(Tab12::mask(A21), 1 << 0);

        assert_eq!(Tab12::lookup(0), 0b0011);
        assert_eq!(Tab12::lookup(1), 0b0110);
        assert_eq!(Tab12::lookup(2), 0b0001);
        assert_eq!(Tab12::lookup(3), 0b0000);

        assert!(Tab12::check_at(0, A11));
        assert!(Tab12::check_at(0, A21));
        assert!(!Tab12::check_at(0, A12));

        assert!(Tab12::check_at(1, A11));
        assert!(Tab12::check_at(1, A12));
        assert!(!Tab12::check_at(1, A21));

        assert!(Tab12::check_at(2, A21));
        assert!(!Tab12::check_at(2, A11));
        assert!(!Tab12::check_at(2, A12));

        assert!(!Tab12::check_at(3, A21));
        assert!(!Tab12::check_at(3, A11));
        assert!(!Tab12::check_at(3, A12));
    }

    #[test]
    fn merge_tab1_with_tab2_with_tab3() {
        assert_eq!(Tab123::len(), 4);

        assert_eq!(Tab123::mask(A11), 1 << 0 + 2);
        assert_eq!(Tab123::mask(A12), 1 << 1 + 2);
        assert_eq!(Tab123::mask(A21), 1 << 0 + 1);
        assert_eq!(Tab123::mask(A31), 1 << 0 + 0);

        assert_eq!(Tab123::lookup(0), 0b0111);
        assert_eq!(Tab123::lookup(1), 0b1100);
        assert_eq!(Tab123::lookup(2), 0b0010);
        assert_eq!(Tab123::lookup(3), 0b0001);

        assert!(Tab123::check_at(0, A11));
        assert!(!Tab123::check_at(0, A12));
        assert!(Tab123::check_at(0, A21));
        assert!(Tab123::check_at(0, A31));

        assert!(Tab123::check_at(1, A11));
        assert!(Tab123::check_at(1, A12));
        assert!(!Tab123::check_at(1, A21));
        assert!(!Tab123::check_at(1, A31));

        assert!(Tab123::check_at(2, A21));
        assert!(!Tab123::check_at(2, A11));
        assert!(!Tab123::check_at(2, A12));
        assert!(!Tab123::check_at(2, A31));

        assert!(!Tab123::check_at(3, A21));
        assert!(!Tab123::check_at(3, A11));
        assert!(!Tab123::check_at(3, A12));
        assert!(Tab123::check_at(3, A31));
    }

    #[test]
    fn dyn_accessor_any() {
        let acc = Any::new(A11) | A21;
        assert!(Tab12::check_at(0, acc));
        assert!(Tab12::check_at(1, acc));
        assert!(Tab12::check_at(2, acc));
        assert!(!Tab12::check_at(3, acc));
    }

    #[test]
    fn dyn_accessor_all() {
        let acc = All::new(A11) & A21;
        assert!(Tab12::check_at(0, acc));
        assert!(!Tab12::check_at(1, acc));
        assert!(!Tab12::check_at(2, acc));
        assert!(!Tab12::check_at(3, acc));
    }

    mod merge_into_bigger_cell_type {
        new_table! {
            flags { F1=F1, F2=F2, F3=F3, F4=F4, F5=F5 }
            struct Table1 {
                static data: [u8; 3] = [F1, F2|F3, F4|F5];
            }
        }

        new_table! {
            flags { E1=E1, E2=E2, E3=E3, E4=E4, E5=E5 }
            struct Table2 {
                static data: [u8; 3] = [E1|E2, E1, E3|E4|E5];
            }
        }

        merge_tables! {
            struct Table3 {
                static data: [u16; 3]
                    = Table1 { F1, F2, F3, F4, F5 }
                    + Table2 { E1, E2, E3, E4, E5 };
            }
        }

        #[test]
        fn to_larger_merge_was_succesfull() {
            use ::Table;
            assert_eq!(Table3::mask(F1), 1 << 0 + 5);
            assert_eq!(Table3::mask(F2), 1 << 1 + 5);
            assert_eq!(Table3::mask(F3), 1 << 2 + 5);
            assert_eq!(Table3::mask(F4), 1 << 3 + 5);
            assert_eq!(Table3::mask(F5), 1 << 4 + 5);
            assert_eq!(Table3::mask(E1), 1 << 0 + 0);
            assert_eq!(Table3::mask(E2), 1 << 1 + 0);
            assert_eq!(Table3::mask(E3), 1 << 2 + 0);
            assert_eq!(Table3::mask(E4), 1 << 3 + 0);
            assert_eq!(Table3::mask(E5), 1 << 4 + 0);
        }
    }

    mod merge_into_smaller {

        new_table! {
            flags {E1=E1, E2=E2, E3=E3, E4=E4, E5=E5 , E6=E6, E7=E7, E8=E8, E9=E9 }
            struct Table2 {
                static data: [u16; 3] = [E1|E2, E6|E7|E8|E9 , E3|E4|E5];
            }
        }


        //FIXME: does not compile but should not...
        // The all indices etc. are found at compiler time, so the compiler does
        // know that E9 exceeds u8, it just kinda does not check this as there
        // is nothing like a "compiler-time safe numcast as"...
        merge_tables! {
            struct Table2v2 {
                static data: [u8; 3] = Table2 { E1, E2, E3, E4, E5, E6, E7, E8, E9 };
            }
        }

        merge_tables! {
            struct Table2v3 {
                static data: [u8; 3] = Table2 { E1, E2 };
            }
        }
    }

}