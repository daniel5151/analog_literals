# Multi-Dimensional Analog Literals in Rust

[![](http://meritbadge.herokuapp.com/analog_literals)](https://crates.io/crates/analog_literals)
[![](https://docs.rs/analog_literals/badge.svg)](https://docs.rs/analog_literals)


Inspired by the ~~abhorrent~~ incredible
[Multi-Dimensional Analog Literals](http://www.eelis.net/C++/analogliterals.xhtml)
library created by Eelis back in 2006, I am proud to present
`analog_literal!` - a Rust macro that enables embedding analog literals
directly into Rust code.

Just take a look at this incredible set of features!

- Support for every physical dimension from 0 to N (for all N <= 3)!
- Strict syntax checking - no mismatched widths, lengths, or heights!
- Entirely safe Rust, enforced by `#[deny(unsafe_code)]`!
- Entirely `const` evaluatable - use analog literals in any `const` context!
- `#![no_std]` by default, _without_ relying on `alloc`!
- Dependency Free!
- Open source under a _very_ permissive license!
- Made with love, passion, and a disregard of all things holy!

# Modes of Operation

The single `analog_literal!` macro can be used to define 3 different kinds
of analog literals: a 1D line, a 2D rectangle, and a 3D cuboid.

## 1D Lines

A 1D analog literal (known in professional circles as a "line") is perfect
for all those times when you wanted to self-document just how big (or
small!) a scalar was.

Defining them works exactly as you'd expect:

```rust
let mine = analog_literal! { +--------------------+ };
let yours = analog_literal! { +----------+ };
assert!(mine > yours);
```

1D analog literals can also be defined using alternative `I` terminators,
for that classic 3rd grade math class feeling:

```rust
assert_eq!(
      analog_literal! { I----I }
    + analog_literal! { I------I },
/*    ---------------------------------------------- */
      analog_literal! { I----------I },
);
```

Heck, you can even use them inline in `const` contexts to quickly and easy
visualize just how large of an array you're allocating!

```rust
let buf = [0; analog_literal!{ +--------------------------------+ }];
```

Educational!

## 2D Rectangles

While there are plenty of Rust libraries out there trying to provide a
elegant, cross-platform solution to writing GUIs, not a single one has a
good way of _visually_ representing the GUI that the code is trying to
represent.

Well, with a bit of help from 2D analog literals (sometimes referred to by
seasoned mathematicians as "rectangles"), these libraries can be
_turbocharged_ with true WYSIWYG capabilities!

```rust
const MODAL_POPUP: Rectangle = analog_literal! {
    +----------------------------------------------------------+
    |                                                          |
    |       /*----------------------------------------*/       |
    |       /* Do you accept the terms and conditions */       |
    |       /* and agree to sell your eternal soul to */       |
    |       /*        the dark lord beelzebub?        */       |
    |       /*----------------------------------------*/       |
    |                                                          |
    |   /* Yes */                             /* Also Yes */   |
    |                                                          |
    +----------------------------------------------------------+
};

assert_eq!(MODAL_POPUP.w, 29);
assert_eq!(MODAL_POPUP.h, 9);

assert_eq!(MODAL_POPUP.area(), 261);
```

The possibilities are endless!

## 3D Cuboids

Say you're trying to solve the [packing problem](https://en.wikipedia.org/wiki/Bin_packing_problem)
for fitting the largest number of dogecoin mining rigs in your 1.5 bedroom
SF "apartment". A 1x programmer would probably just use a couple of boring
'ol cOnStAnTs to define the length, width, and height of their dogecoin
miners. Of course, you're not a 1x programmer, you're a _10x programmer_, so
you use a 3D analog literal (colloquially referred to as a "cuboid") to
_visually_ represent the dimensions of your mining rigs:

```rust
const MINING_RIG: Cuboid = analog_literal! {
                     +------------------------------------------+
                    /                                          /|
                   /                                          / +
                  /                   /**/                   / /
                 /                   /**/                   / /
                /                   /**/                   / /
               /                                          / /
              /                                          / /
             /                                          / /
            /                                          / /
           /                                          / /
          /                                          / /
         /                                          / /
        /                                          / /
       /                                          / /
      /                                          / /
     /                                          / /
    +------------------------------------------+ /
    | /* -= dogecoin-one =- */         /*(o)*/ |/
    +------------------------------------------+
};

assert_eq!(MINING_RIG.w, 21);
assert_eq!(MINING_RIG.h, 1);
assert_eq!(MINING_RIG.l, 16);

assert_eq!(MINING_RIG.volume(), 336);
```

Oh yeah, I can taste those DOGE tendies already!

# Putting it all together

Brace yourself, because the following example may very-well shake you to the core:

```rust
use analog_literals::analog_literal as al;

al! {
       +----------+
      /          /|
     /          / |
    /          /  +
   /          /  /
  +----------+  /
  |          | /
  |          |/
  +----------+ }.volume() == al! { +----+
                                   |    |
                                   |    |
                                   |    |
                                   |    |
                                   |    |
                                   +----+ }.area() * al! { +--------+ }
```

# Safety and Syntax Validation

In true Rust fashion, this library strives to be as safe as can be. Not only
is it 100% unsafe code free (enforced by `#![deny(unsafe_code)]` in the
crate root), but it will also catch any malformed analog literals at compile
time!

I'm sure we've all been in this situation: it's 4:55 on a weekday, you're
desperate for another cup of coffee, and you really feel like just packing
it in early. You decide to call it for the day, and leave yourself a comment
to pick up where you left off tomorrow...

```rust,compile_fail
const YET_ANOTHER_RECT: Rectangle = analog_literal! {
    +----------------+
    |                |
    |                |
    +------------+ // TODO: type more `--`...
};
```

You come back in the next morning, and without thinking, you decide to check
in what you had from yesterday into prod. But it's okay, because as the CI
runs on your shoddy code from the day before, the Rust compiler helpfully
spits out a compilation error!

```text
error[E0080]: evaluation of constant value failed
  --> src/lib.rs:364:37
   |
5  |   const YET_ANOTHER_RECT: Rectangle = analog_literal! {
   |  _____________________________________^
6  | |     +----------------+
7  | |     |                |
8  | |     |                |
9  | |     +------------+ // TODO: type more `--`...
10 | | };
   | |_^ attempt to compute `0_usize - 1_usize`, which would overflow
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
```

Look, I never said the error messages would be descriptive, okay...

But hey, would you look at that! With `analog_literal!`, you can rest easy
knowing that _your_ literals are guaranteed to be well formed _From Top to
Bottom, and Left to Right™_

---

So what are you waiting for! Go ahead and add `analog_literals` to your
project _today!_

```toml
[dependencies]
analog_literals = "*"
```

# Final Comments

I'm not gonna lie - I was already half-way done implementing 3D literals
before I'd even thought to check if anyone else had done something as stupid
as this already. Lo and behold, [jswrenn/analit](https://github.com/jswrenn/analit)
beat me to the punch by a whopping 6 years!

A smarter man might have stopped working on his shoddy rewrite and scrapped
his shoddy POC then and there. Instead, stubborn as I am, I decided to push
forward and spent another handful of hours polishing it up, adding syntax
validation, and writing a whole _mess_ of docs.

Do I regret it? Ehh, maybe.

It is funny tho? I mean, I think so.

---

Oh, and lastly: please, _please_ don't use this library in an actual
project.

If I saw something like this while perusing a crate's source code, I'd
probably nope the heck out of there, turn off my computer, and go on a long
walk to contemplate what kind of life choices could have led someone to
thinking that including `analog_literals` in their project would be a good
idea.
