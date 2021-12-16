//! A convenience macro for implementing basic traits.
//!
//! Supported traits:
//!
//! - [`std::fmt::Display`], [`std::fmt::Debug`]
//! - [`std::convert::From`]
//! - [`std::convert::Into`]
//! - [`std::ops::Deref`]
//! - [`std::ops::DerefMut`]
//!
//! # Syntax
//!
//! The [`implem!`] macro expects a sequence of elements of shape
//!
//! ```text
//! $( impl ($type_parameters) )?
//! for $self_type
//! $( where ($where_clauses) )?
//! {
//!     $( $trait_def )*
//! }
//! ```
//!
//! See the [examples](#examples) below for details regarding `$trait_def`initions. Generally
//! speaking, they look like
//!
//! - `$trait_name { |$args| $def }` or
//! - `$trait_name< $t_params_if_any, $assoc_types_if_any = $type > { |$args| $def }`
//!
//! The `|$args| $def` defines the function that needs to be implemented in `$trait_name`. For
//! instance, `from` for [`From`], `deref_mut` for [`std::ops::DerefMut`], or `fmt` for
//! [`std::fmt::Display`].
//!
//! # Examples
//!
//! ## `Display`, `Debug`, `From` and `Into`
//!
//! ```rust
//! # use implem::implem;
//! pub struct MyStruct {
//!     s: String,
//! }
//! implem! {
//!     for MyStruct {
//!         Display {
//!             |&self, fmt| write!(fmt, "{{ s: `{}` }}", self.s)
//!         }
//!         Debug {
//!             |&self, fmt| write!(fmt, "MyStruct {{ s: `{}` }}", self.s)
//!         }
//!         From<String> {
//!             |s| Self { s }
//!         }
//!     }
//!     impl('a) for MyStruct {
//!         From<&'a str> {
//!             |s| Self { s: s.to_string() }
//!         }
//!     }
//!     impl('a) for &'a MyStruct {
//!         Into<&'a str> {
//!             |self| &self.s
//!         }
//!     }
//! }
//! ```
//!
//! ## `Deref` and `DerefMut`
//!
//! ```rust
//! # use implem::implem;
//! pub struct MyStruct1 {
//!     s: String,
//! }
//! implem! {
//!     for MyStruct1 {
//!         Deref<Target = String> {
//!             |&self| &self.s,
//!             // next *optional* line implements `DerefMut` as well
//!             |&mut self| &mut self.s,
//!         }
//!     }
//! }
//!
//! pub struct MyStruct2 {
//!     s: String,
//! }
//! implem! {
//!     for MyStruct2 {
//!         Deref<Target = String> {
//!             |&self| &self.s
//!         }
//!         // explicit `DerefMut` implementation
//!         DerefMut {
//!             |&mut self| &mut self.s
//!         }
//!     }
//! }
//! ```

/// The whole point, see [crate-level documentation][doc] for details.
///
/// [doc]: ./index.html (crate-level documentation)
#[macro_export]
macro_rules! implem {
    {
        $(
            impl ($($t_params:tt)*)
        )?
        for $self_ty:ty
        $(
            where ($($where_clauses:tt)*)
        )? {
            $($stuff:tt)*
        }

        $($tail:tt)*
    } => {
        $crate::internal! {
            @(
                $( $($t_params)* )?
            )(
                $( $($where_clauses)* )?
            )(
                $self_ty
            )
            $($stuff)*
        }
        $crate::implem! { $($tail)* }
    };
    {} => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! internal {
    { @
        ( $($t_params:tt)* )
        ( $($where_clauses:tt)* )
        ($self_ty:ty)
        Display {
            |&$slf:ident, $fmt:pat| $def:expr
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::fmt::Display for $self_ty
        where $($where_clauses)* {
            fn fmt(&$slf, $fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clauses)*)($self_ty)
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clauses:tt)* )
        ($self_ty:ty)
        Debug {
            |&$slf:ident, $fmt:pat| $def:expr
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::fmt::Debug for $self_ty
        where $($where_clauses)* {
            fn fmt(&$slf, $fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clauses)*)($self_ty)
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clauses:tt)* )
        ($self_ty:ty)
        From<$src_ty:ty> {
            |$src:pat| $def:expr $(,)?
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::convert::From<$src_ty> for $self_ty
        where $($where_clauses)* {
            fn from($src: $src_ty) -> Self {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clauses)*)($self_ty)
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clauses:tt)* )
        ($self_ty:ty)
        Into<$tgt_ty:ty> {
            |$slf:ident| $def:expr $(,)?
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::convert::Into<$tgt_ty> for $self_ty
        where $($where_clauses)* {
            fn into($slf) -> $tgt_ty {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clauses)*)($self_ty)
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clause:tt)* )
        ($self_ty:ty)
        Deref<Target = $tgt_ty:ty> {
            |&$slf:ident| $def:expr
            $(
                , |&mut $slf_mut:ident| $def_mut:expr
            )?
            $(,)?
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::ops::Deref for $self_ty
        where $($where_clause)* {
            type Target = $tgt_ty;
            fn deref(&$slf) -> &$tgt_ty {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clause)*)($self_ty)
            $(
                DerefMut { |&mut $slf_mut| $def_mut }
            )?
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clause:tt)* )
        ($self_ty:ty)
        DerefMut {
            |&mut $slf:ident| $def:expr $(,)?
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::ops::DerefMut for $self_ty
        where $($where_clause)* {
            fn deref_mut(&mut $slf) -> &mut <Self as std::ops::Deref>::Target {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clause)*)($self_ty)
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clause:tt)* )
        ($self_ty:ty)
        Index<$idx_ty:ty, Output = $out_ty:ty> {
            |&$slf:ident, $idx:pat| $def:expr
            $(
                , |&mut $slf_mut:ident, $idx_mut:pat| $def_mut:expr
            )?
            $(,)?
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::ops::Index<$idx_ty> for $self_ty
        where $($where_clause)* {
            type Output = $out_ty;
            fn index(&$slf, $idx: $idx_ty) -> &Self::Output {
                $def
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clause)*)($self_ty)
            $(
                IndexMut<$idx_ty> { |&mut $slf_mut, $idx_mut| $def_mut }
            )?
            $($tail)*
        }
    };
    { @
        ( $($t_params:tt)* )
        ( $($where_clause:tt)* )
        ($self_ty:ty)
        IndexMut<$idx_ty:ty> {
            |&mut $slf:ident, $idx:pat| $def_mut:expr
            $(,)?
        }
        $($tail:tt)*
    } => {
        impl<$($t_params)*> std::ops::IndexMut<$idx_ty> for $self_ty
        where $($where_clause)* {
            fn index_mut(&mut $slf, $idx: $idx_ty) -> &mut Self::Output {
                $def_mut
            }
        }
        $crate::internal! {
            @($($t_params)*)($($where_clause)*)($self_ty)
            $($tail)*
        }
    };

    { @
        ( $($t_params:tt)* )
        ( $($where_clause:tt)* )
        ($self_ty:ty)

        $unk:ident

        $($stuff:tt)*
    } => {
        compile_error!(concat!(
            "expected known trait, got `", stringify!($unk), "`"
        ))
    };

    {} => {};
    { @
        ($($t_params:tt)*)
        ($($where_clause:tt)*)
        ($self_ty:ty)
    } => {};
}
