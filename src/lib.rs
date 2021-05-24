//! Inspired by the ~~abhorrent~~ incredible
//! [Multi-Dimensional Analog Literals](http://www.eelis.net/C++/analogliterals.xhtml)
//! library created by Eelis back in 2006, I am proud to present
//! `analog_literal!` - a Rust macro that enables embedding analog literals
//! directly into Rust code.
//!
//! Just take a look at this incredible set of features!
//!
//! - Support for every physical dimension from 0 to N (for all N <= 3)!
//! - Entirely safe Rust, enforced by `#[deny(unsafe_code)]`!
//! - Entirely `const` evaluatable - use analog literals in any `const` context!
//! - `#![no_std]` by default, _without_ relying on `alloc`!
//! - Dependency Free!
//! - Open source under a _very_ permissive license!
//! - Made with love, passion, and a disregard of all things holy!
//!
//! # Modes of Operation
//!
//! The single `analog_literal!` macro can be used to define 3 different kinds
//! of analog literals: a 1D line, a 2D rectangle, and a 3D cuboid.
//!
//! ## 1D Lines
//!
//! A 1D analog literal (known in professional circles as a "line") is perfect
//! for all those times when you wanted to self-document just how big (or
//! small!) a scalar was.
//!
//! Defining them works exactly as you'd expect:
//!
//! ```rust
//! # use analog_literals::{analog_literal};
//! let mine = analog_literal! { +--------------------+ };
//! let yours = analog_literal! { +----------+ };
//! assert!(mine > yours);
//! ```
//!
//! 1D analog literals can also be defined using alternative `I` terminators,
//! for that classic 3rd grade math class feeling:
//!
//! ```rust
//! # use analog_literals::{analog_literal};
//! assert_eq!(
//!       analog_literal! { I----I }
//!     + analog_literal! { I------I },
//! /*    ---------------------------------------------- */
//!       analog_literal! { I----------I },
//! );
//! ```
//!
//! Heck, you can even use them inline in `const` contexts to quickly and easy
//! visualize just how large of an array you're allocating!
//!
//! ```rust
//! # use analog_literals::{analog_literal};
//! let buf = [0; analog_literal!{ +--------------------------------+ }];
//! ```
//!
//! Educational!
//!
//! ## 2D Rectangles
//!
//! While there are plenty of Rust libraries out there trying to provide a
//! elegant, cross-platform solution to writing GUIs, not a single one has a
//! good way of _visually_ representing the GUI that the code is trying to
//! represent.
//!
//! Well, with a bit of help from 2D analog literals (sometimes referred to by
//! seasoned mathematicians as "rectangles"), these libraries can be
//! _turbocharged_ with true WYSIWYG capabilities!
//!
//! ```rust
//! # use analog_literals::{analog_literal, Rectangle};
//! const MODAL_POPUP: Rectangle = analog_literal! {
//!     +----------------------------------------------------------+
//!     |                                                          |
//!     |       /*----------------------------------------*/       |
//!     |       /* Do you accept the terms and conditions */       |
//!     |       /* and agree to sell your eternal soul to */       |
//!     |       /*        the dark lord beelzebub?        */       |
//!     |       /*----------------------------------------*/       |
//!     |                                                          |
//!     |   /* Yes */                             /* Also Yes */   |
//!     |                                                          |
//!     +----------------------------------------------------------+
//! };
//!
//! assert_eq!(MODAL_POPUP.w, 29);
//! assert_eq!(MODAL_POPUP.h, 9);
//!
//! assert_eq!(MODAL_POPUP.area(), 261);
//! ```
//!
//! The possibilities are endless!
//!
//! ## 3D Cuboids
//!
//! Say you're trying to solve the [packing problem](https://en.wikipedia.org/wiki/Bin_packing_problem)
//! for fitting the largest number of dogecoin mining rigs in your 1.5 bedroom
//! SF "apartment". A 1x programmer would probably just use a couple of boring
//! 'ol cOnStAnTs to define the length, width, and height of their dogecoin
//! miners. Of course, you're not a 1x programmer, you're a _10x programmer_, so
//! you use a 3D analog literal (colloquially referred to as a "cuboid") to
//! _visually_ represent the dimensions of your mining rigs:
//!
//! ```rust
//! # use analog_literals::{analog_literal, Cuboid};
//! const MINING_RIG: Cuboid = analog_literal! {
//!                      +------------------------------------------+
//!                     /                                          /|
//!                    /                                          / +
//!                   /                   /**/                   / /
//!                  /                   /**/                   / /
//!                 /                   /**/                   / /
//!                /                                          / /
//!               /                                          / /
//!              /                                          / /
//!             /                                          / /
//!            /                                          / /
//!           /                                          / /
//!          /                                          / /
//!         /                                          / /
//!        /                                          / /
//!       /                                          / /
//!      /                                          / /
//!     +------------------------------------------+ /
//!     | /* -= dogecoin-one =- */         /*(o)*/ |/
//!     +------------------------------------------+
//! };
//!
//! assert_eq!(MINING_RIG.w, 21);
//! assert_eq!(MINING_RIG.h, 1);
//! assert_eq!(MINING_RIG.l, 16);
//!
//! assert_eq!(MINING_RIG.volume(), 336);
//! ```
//!
//! Oh yeah, I can taste those DOGE tendies already!
//!
//! # Safety and Syntax Validation
//!
//! In true Rust fashion, this library strives to be as safe as can be. Not only
//! is it 100% unsafe code free (enforced by `#![deny(unsafe_code)]` in the
//! crate root), but it will also catch any malformed analog literals at compile
//! time!
//!
//! I'm sure we've all been in this situation: it's 4:55 on a weekday, you're
//! desperate for another cup of coffee, and you really feel like just packing
//! it in early. You decide to call it for the day, and leave yourself a comment
//! to pick up where you left off tomorrow...
//!
//! ```rust,compile_fail
//! # use analog_literals::{analog_literal, Rectangle};
//! const YET_ANOTHER_RECT: Rectangle = analog_literal! {
//!     +----------------+
//!     |                |
//!     |                |
//!     +------------+ // TODO: type more `--`...
//! };
//! ```
//!
//! You come back in the next morning, and without thinking, you decide to check
//! in what you had from yesterday into prod. But it's okay, because as the CI
//! runs on your shoddy code from the day before, the Rust compiler helpfully
//! spits out a compilation error!
//!
//! ```text
//! error[E0080]: evaluation of constant value failed
//!   --> src/lib.rs:364:37
//!    |
//! 5  |   const YET_ANOTHER_RECT: Rectangle = analog_literal! {
//!    |  _____________________________________^
//! 6  | |     +----------------+
//! 7  | |     |                |
//! 8  | |     |                |
//! 9  | |     +------------+ // TODO: type more `--`...
//! 10 | | };
//!    | |_^ attempt to compute `0_usize - 1_usize`, which would overflow
//!    |
//!    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
//! ```
//!
//! Look, I never said the error messages would be descriptive, okay...
//!
//! But hey, would you look at that! With `analog_literal!`, you can rest easy
//! knowing that _your_ literals are guaranteed to be well formed _From Top to
//! Bottom, and Left to Right™_
//!
//! * * *
//!
//! So what are you waiting for! Go ahead and add `analog_literals` to your
//! project _today!_
//!
//! ```toml
//! [dependencies]
//! analog_literals = "*"
//! ```
//!
//! # Final Comments
//!
//! I'm not gonna lie - I was already half-way done implementing 3D literals
//! before I'd even thought to check if anyone else had done something as stupid
//! as this already. Lo and behold, [jswrenn/analit](https://github.com/jswrenn/analit)
//! beat me to the punch by a whopping 6 years!
//!
//! A smarter man might have stopped working on his shoddy rewrite and scrapped
//! his shoddy POC then and there. Instead, stubborn as I am, I decided to push
//! forward and spent another handful of hours polishing it up, adding syntax
//! validation, and writing a whole _mess_ of docs.
//!
//! Do I regret it? Ehh, maybe.
//!
//! It is funny tho? I mean, I think so.
//!
//! * * *
//!
//! Oh, and lastly: please, _please_ don't use this library in an actual
//! project.
//!
//! If I saw something like this while perusing a crate's source code, I'd
//! probably nope the heck out of there, turn off my computer, and go on a long
//! walk to contemplate what kind of life choices could have led someone to
//! thinking that including `analog_literals` in their project would be a good
//! idea.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

/// A 1D Line of a particular length.
///
/// I'll be honest, I thought it'd be funnier to have the `analog_literal!`
/// macro just return a untyped number instead of a structured `Line` struct.
///
/// I'm just including this struct as part of the docs for completeness's sake,
/// since I know that if I don't, _someone_ is going to bring it up.
pub struct Line(pub usize);

/// A 2D Rectangle with a certain width and height.
///
/// Note that `Rectangle` does not implement `Eq` nor `Ord`, as it would be rude
/// to assume that two literals are the same just because they are rotationally
/// transformed.
///
/// # Example
///
/// Ever needed to define a particular aspect ratio? Well, why not represent
/// that aspect ratio visually using a analog literal rectangle instead!
///
/// WARNING: aspect ratios may look a bit "off" depending on the font being
/// used to display the source code. This is normal, and should not affect the
/// correctness of the analog literal.
///
/// ```rust
/// # use analog_literals::analog_literal;
/// let aspect_ratio = analog_literal! {
///     +--------+
///     |        |
///     |        |
///     |        |
///     +--------+
/// };
///
/// assert_eq!(aspect_ratio.w, 4);
/// assert_eq!(aspect_ratio.h, 3);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    /// Width of the literal (counts how many `--` wide the analog literal is)
    pub w: usize,
    /// Height of the literal (counts how many `|` tall the analog literal is)
    pub h: usize,
}

impl Rectangle {
    /// Return the area of the rectangle.
    ///
    /// This is a highly complex operation, and has been aggressively optimized
    /// for maximum performance. Gaze upon the implementation... if you dare.
    pub const fn area(&self) -> usize {
        self.w * self.h
    }
}

/// A 3D Cuboid with a certain width, height, and length.
///
/// # Example
///
/// Say you're trying to solve the [packing problem](https://en.wikipedia.org/wiki/Bin_packing_problem)
/// for fitting the largest number of dogecoin mining rigs in your 1.5 bedroom
/// SF "apartment". A 1x programmer would probably just use a couple of boring
/// 'ol cOnStAnTs to define the length, width, and height of their dogecoin
/// miners. Of course, you're not a 10x programmer, so you use a 3D analog
/// literal to _visually_ represent the dimensions of your mining rigs:
///
/// ```rust
/// # use analog_literals::{analog_literal, Cuboid};
/// const MINING_RIG: Cuboid = analog_literal! {
///                      +------------------------------------------+
///                     /                                          /|
///                    /                                          / +
///                   /                   /**/                   / /
///                  /                   /**/                   / /
///                 /                   /**/                   / /
///                /                                          / /
///               /                                          / /
///              /                                          / /
///             /                                          / /
///            /                                          / /
///           /                                          / /
///          /                                          / /
///         /                                          / /
///        /                                          / /
///       /                                          / /
///      /                                          / /
///     +------------------------------------------+ /
///     | /* -= dogecoin one =- */         /*(o)*/ |/
///     +------------------------------------------+
/// };
///
/// assert_eq!(MINING_RIG.w, 21);
/// assert_eq!(MINING_RIG.h, 1);
/// assert_eq!(MINING_RIG.l, 16);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Cuboid {
    /// Width of the literal (counts how many `--` wide the analog literal is)
    pub w: usize,
    /// Height of the literal (counts how many `|` tall the analog literal is)
    pub h: usize,
    /// Length of the literal (counts how many `/` deep the analog literal is)
    pub l: usize,
}

impl Cuboid {
    /// Return the volume of the cuboid.
    ///
    /// Note that `Cuboid` does not implement `Eq` nor `Ord`, as it would be
    /// rude to assume that two literals are the same just because they are
    /// rotationally transformed.
    ///
    /// Just like [`Rectangle::area()`], the underlying implementation of this
    /// method is highly complex and aggressively optimized. You could take a
    /// look, but you'll be hard pressed to make heads or tails of it. You've
    /// been warned.
    pub const fn volume(&self) -> usize {
        self.w * self.h * self.l
    }

    /// Returns a [`Rectangle`] with the same dimensions as the top of the
    /// Cubiod.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use analog_literals::{analog_literal, Cuboid};
    ///
    /// pub const CUBE_5_BY_2_BY_4: Cuboid = analog_literal! {
    ///         +----------+
    ///        /          /|
    ///       /          / |
    ///      /          /  +
    ///     /          /  /
    ///    +----------+  /
    ///    |          | /
    ///    |          |/
    ///    +----------+
    /// };
    /// let top = analog_literal! {
    ///     +----------+
    ///     |          |
    ///     |          |
    ///     |          |
    ///     |          |
    ///     +----------+
    /// };
    ///
    /// assert_eq!(CUBE_5_BY_2_BY_4.top().w, top.w);
    /// assert_eq!(CUBE_5_BY_2_BY_4.top().h, top.h);
    /// ```
    pub const fn top(&self) -> Rectangle {
        Rectangle {
            w: self.w,
            h: self.l,
        }
    }

    /// Returns a [`Rectangle`] with the same dimensions as the side of the
    /// Cubiod.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use analog_literals::{analog_literal, Cuboid};
    ///
    /// pub const CUBE_5_BY_2_BY_4: Cuboid = analog_literal! {
    ///         +----------+
    ///        /          /|
    ///       /          / |
    ///      /          /  +
    ///     /          /  /
    ///    +----------+  /
    ///    |          | /
    ///    |          |/
    ///    +----------+
    /// };
    /// let side = analog_literal! {
    ///     +--------+
    ///     |        |
    ///     |        |
    ///     +--------+
    /// };
    ///
    /// assert_eq!(CUBE_5_BY_2_BY_4.side().w, side.w);
    /// assert_eq!(CUBE_5_BY_2_BY_4.side().h, side.h);
    /// ```
    pub const fn side(&self) -> Rectangle {
        Rectangle {
            w: self.l,
            h: self.h,
        }
    }

    /// Returns a [`Rectangle`] with the same dimensions as the front of the
    /// Cubiod.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use analog_literals::{analog_literal, Cuboid};
    ///
    /// pub const CUBE_5_BY_2_BY_4: Cuboid = analog_literal! {
    ///         +----------+
    ///        /          /|
    ///       /          / |
    ///      /          /  +
    ///     /          /  /
    ///    +----------+  /
    ///    |          | /
    ///    |          |/
    ///    +----------+
    /// };
    /// let front = analog_literal! {
    ///     +----------+
    ///     |          |
    ///     |          |
    ///     +----------+
    /// };
    ///
    /// assert_eq!(CUBE_5_BY_2_BY_4.front().w, front.w);
    /// assert_eq!(CUBE_5_BY_2_BY_4.front().h, front.h);
    /// ```
    pub const fn front(&self) -> Rectangle {
        Rectangle {
            w: self.w,
            h: self.h,
        }
    }
}

/// The star of the show: a macro to embed analog literals into otherwise boring
/// Rust source code.
///
/// Check out the crate-level documentation for more info.
#[macro_export]
macro_rules! analog_literal {
    (I $($tail:tt)+) => {
        0 + $crate::__analog_literal!(@1D $($tail)+)
    };

    (II) => {
        0
    };

    (+ $($tail:tt)+) => {
        $crate::__analog_literal! {
            @2D_TOP (
                { 0 },
                { 0 },
            )

            ; $($tail)+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __analog_literal {
    //========================================================================//
    //                              Entrypoints                               //
    //========================================================================//

    (I $($tail:tt)+) => {
        0 + $crate::__analog_literal!(@1D $($tail)+)
    };

    // edge case
    (II) => {
        0
    };

    (+ $($tail:tt)+) => {
        $crate::__analog_literal! {
            @2D_TOP (
                { 0 },
                { 0 },
            )

            ; $($tail)+
        }
    };

    //========================================================================//
    //                                Utilities                               //
    //========================================================================//

    (@const_assert $x:expr $(,)?) => {
        #[allow(unknown_lints)]
        const _: [(); 0 - !{
            const ASSERT: bool = $x;
            ASSERT
        } as usize] = [];
    };

    //========================================================================//
    //                                   1D                                   //
    //========================================================================//

    (@1D -- $($tail:tt)+) => {
        1 + $crate::__analog_literal!(@1D $($tail)+)
    };

    (@1D I) => {
        0
    };

    //========================================================================//
    //                                   2D                                   //
    //========================================================================//

    (
        @2D_TOP (
            { $($w:tt)+ },
            { $($h:tt)+ },
        )

        ; -- $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @2D_TOP (
                { $($w)+ + 1 },
                { $($h)+ },
            )

            ; $($tail)+
        }
    };

    (
        @2D_TOP (
            { $($w:tt)+ },
            { $($h:tt)+ },
        )

        ; + // degenerate case of a 1d literal
    ) => {
        $($w)+
    };

    (
        @2D_TOP (
            { $($w:tt)+ },
            { $($h:tt)+ },
        )

        ; + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @2D_MID (
                { $($w)+ },
                { $($h)+ },
            )

            ; $($tail)+
        }
    };

    (
        @2D_MID (
            { $($w:tt)+ },
            { $($h:tt)+ },
        )

        ; | | $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @2D_MID (
                { $($w)+ },
                { 1 + $($h)+ },
            )

            ; $($tail)+
        }
    };

    (
        @2D_MID (
            { $($w:tt)+ },
            { $($h:tt)+ },
        )

        ; + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @2D_BOTTOM (
                { $($w)+ },
                { $($h)+ },
                { 0 },
            )

            ; $($tail)+
        }
    };

    (
        @2D_BOTTOM (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($bottom_w:tt)+ },
        )

        ; -- $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @2D_BOTTOM (
                { $($w)+ },
                { $($h)+ },
                { $($bottom_w)+ + 1 },
            )

            ; $($tail)+
        }
    };

    (
        @2D_BOTTOM (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($bottom_w:tt)+ },
        )

        ; +
    ) => {{
        $crate::__analog_literal!(@const_assert { $($w)+ } == { $($bottom_w)+ });
        $crate::Rectangle {
            w: $($w)+,
            h: $($h)+,
        }
    }};

    //========================================================================//
    //                                   3D                                   //
    //========================================================================//

    (
        @2D_MID (
            { $($w:tt)+ },
            { $($h:tt)+ },
        )

        ; / / | $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_TOP_l_h (
                { $($w)+ },
                { 1 },
                { 1 },

                { 0 },
                { 0 },
                { 0 },
                { 0 },
            )

            ; $($tail)+
        }
    };

    (
        @3D_TOP_l_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; / / | $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_TOP_l_h (
                { $($w)+ },
                { 1 + $($h)+ },
                { 1 + $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_TOP_l_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; / / + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_TOP_l_BOTTOM_l (
                { $($w)+ },
                { $($h)+ },
                { 1 + $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_TOP_l_BOTTOM_l (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; / / / $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_TOP_l_BOTTOM_l (
                { $($w)+ },
                { $($h)+ },
                { 1 + $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { 1 + $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_TOP_l_BOTTOM_l (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_MID_w (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_TOP_l_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_MID_w (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_MID_w (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; -- $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_MID_w (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { 1 + $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_MID_w (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; + + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM_h (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_MID_w (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; + | $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM_h (
                { $($w)+ },
                { 1 + $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_MID_w (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; + / $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM_h (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { 1 + $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_BOTTOM_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; | | + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM_h (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { 1 + $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_BOTTOM_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; | | / $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM_h (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { 1 + $($bottom_h)+ },
                { 1 + $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_BOTTOM_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; | | | $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM_h (
                { $($w)+ },
                { 1 + $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { 1 + $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_BOTTOM_h (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; + $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_BOTTOM (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; -- $($tail:tt)+
    ) => {
        $crate::__analog_literal! {
            @3D_BOTTOM (
                { $($w)+ },
                { $($h)+ },
                { $($l)+ },

                { $($mid_w)+ },
                { 1 + $($bottom_w)+ },
                { $($bottom_h)+ },
                { $($bottom_l)+ },
            )

            ; $($tail)+
        }
    };

    (
        @3D_BOTTOM (
            { $($w:tt)+ },
            { $($h:tt)+ },
            { $($l:tt)+ },

            { $($mid_w:tt)+ },
            { $($bottom_w:tt)+ },
            { $($bottom_h:tt)+ },
            { $($bottom_l:tt)+ },
        )

        ; +
    ) => {
        {
            $crate::__analog_literal!(@const_assert { $($w)+ } == { $($mid_w)+ });
            $crate::__analog_literal!(@const_assert { $($w)+ } == { $($bottom_w)+ });
            $crate::__analog_literal!(@const_assert { $($h)+ } == { $($bottom_h)+ });
            $crate::__analog_literal!(@const_assert { $($l)+ } == { $($bottom_l)+ });

            $crate::Cuboid {
                w: $($w)+,
                l: $($l)+,
                h: $($h)+,
            }
        }
    };

    // (
    //     $val,
    //     "---",
    //     { $($w)+ },
    //     { $($h)+ },
    //     { $($l)+ },
    //     "---",
    //     { $($mid_w)+ },
    //     { $($bottom_w)+ },
    //     { $($bottom_h)+ },
    //     { $($bottom_l)+ },
    // )
}

#[cfg(test)]
mod test {
    use super::*;

    // works in const contexts as well!

    pub const RECT_2_BY_3: Rectangle = analog_literal! {
        +----+
        |    |
        |    |
        |    |
        +----+
    };

    pub const CUBE_5_BY_2_BY_4: Cuboid = analog_literal! {
             +----------+
            /          /|
           /          / |
          /          /  +
         /          /  /
        +----------+  /
        |          | /
        |          |/
        +----------+
    };

    #[allow(dead_code)]
    pub const CUBE_4_BY_2_BY_2: Cuboid = analog_literal! {
           +--------+
          /        /|
         /        / |
        +--------+  +
        |        | /
        |        |/
        +--------+
    };

    #[allow(dead_code)]
    pub const CUBE_4_BY_5_BY_1: Cuboid = analog_literal! {
          +--------+
         /        /|
        +--------+ |
        |        | |
        |        | |
        |        | |
        |        | +
        |        |/
        +--------+
    };

    // make it just a bit larger and you'll hit the recursion limit lol
    pub const CHONKER: Cuboid = analog_literal! {
              +------------------------------------------------------------+
             /                                                            /|
            /                                                            / |
           /                                                            /  |
          /                /* Macros were a mistake */                 /   |
         /                                                            /    |
        /                                                            /     |
       /                                                            /      |
      +------------------------------------------------------------+       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       |
      |                                                            |       +
      |                                                            |      /
      |                                                            |     /
      |                                                            |    /
      |                                                            |   /
      |                                                            |  /
      |                                                            | /
      |                                                            |/
      +------------------------------------------------------------+
    };

    #[test]
    fn main() {
        assert_eq!(analog_literal! { II }, 0);
        assert_eq!(analog_literal! { I--I }, 1);
        assert_eq!(analog_literal! { I----I }, 2);
        assert_eq!(analog_literal! { I------I }, 3);

        // can also use + as terminator
        assert_eq!(analog_literal! { ++ }, 0);
        assert_eq!(analog_literal! { +--+ }, 1);
        assert_eq!(analog_literal! { +----+ }, 2);
        assert_eq!(analog_literal! { +------+ }, 3);

        assert_eq!(
            RECT_2_BY_3.area(),
            analog_literal! { +----+ } * analog_literal! { +------+ }
        );

        assert_eq!(CUBE_5_BY_2_BY_4.volume(), 40);

        eprintln!("{:?}", CHONKER);
    }

    macro_rules! const_assert {
        ($x:expr $(,)?) => {
            #[allow(unknown_lints)]
            const _: [(); 0 - !{
                const ASSERT: bool = $x;
                ASSERT
            } as usize] = [];
        };
    }

    // and of course, you can do const arithmetic
    const_assert! {
        analog_literal! {
                +----------+
               /          /|
              /          / |
             /          /  +
            /          /  /
           +----------+  /
           |          | /
           |          |/
           +----------+ }.volume() == analog_literal! { +----+
                                                        |    |
                                                        |    |
                                                        |    |
                                                        |    |
                                                        |    |
                                                        +----+ }.area() * analog_literal! { +--------+ }
    }
}
