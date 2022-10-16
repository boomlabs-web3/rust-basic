# Part 4

### Generic structs

<details>
<summary> Explanation </summary>
<div markdown="1"> <blockquote> 

We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types. Let’s first look at how to define functions, structs, enums, and methods using generics. Then we’ll discuss how generics affect code performance.

- Function Definitions
    
```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
    
- Struct Definitions
    
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
    
- Enum Definitions
    
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
    
- Method Definitions
    
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-01-syntax.html)

- [Rust By Example](https://doc.rust-lang.org/rust-by-example/generics.html)

- [Rust By Example](https://doc.rust-lang.org/rust-by-example/generics/impl.html)

</div></details>
<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “generics 1, 2” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/generics/`

and you can verify your work by running `rustlings run generics1` command.
</blockquote></div></details>

### Multivariable generics

<details>
<summary> Explanation </summary>
<div markdown="1"> <blockquote> 

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters. For example, we change the definition of `Point` to be generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`.

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures. Above code snippet uses the generic types `X1` and `Y1` for the `Point` struct and `X2` `Y2` for the `mixup` method signature to make the example clearer. The method creates a new `Point` instance with the `x` value from the `self` `Point` (of type `X1`) and the `y` value from the passed-in `Point` (of type `Y2`).

In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`) and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct that has a string slice for `x` (with value `"Hello"`) and a `char` for `y` (with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`, which will have an `i32` for `x`, because `x` came from `p1`. The `p3` variable will have a `char` for `y`, because `y` came from `p2`. The `println!` macro call will print `p3.x = 5, p3.y = c`.

The purpose of this example is to demonstrate a situation in which some generic parameters are declared with `impl` and some are declared with the method definition. Here, the generic parameters `X1` and `Y1` are declared after `impl` because they go with the struct definition. The generic parameters `X2` and `Y2` are declared after `fn mixup`, because they’re only relevant to the method.

</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [Rust By Example](https://doc.rust-lang.org/rust-by-example/generics/multi_bounds.html)
</div></details>

### Generic functions

</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/generics.html#generic-functions)

- [The Rust Reference](https://doc.rust-lang.org/reference/items/functions.html?highlight=generic#generic-functions)
</div></details>

### Generic lifetimes

</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#generic-lifetimes-in-functions)

- [The Rust Programming Language](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/lifetimes.html#lifetimes)

- [Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html)
</div></details>

### Type constraints, Boxed dyn

<details>
<summary> `Box<T>` Explanation </summary>
<div markdown="1"> <blockquote> 

The most straightforward smart pointer is a *box*, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
</blockquote></div></details>
<details>

<summary>`dyn` keyword Explanation </summary>
<div markdown="1"> <blockquote> 

The `dyn` keyword is used to highlight that calls to methods on the associated `Trait` are [dynamically dispatched](https://en.wikipedia.org/wiki/Dynamic_dispatch). To use the trait this way, it must be ‘object safe’.

Unlike generic parameters or `impl Trait`, the compiler does not know the concrete type that is being passed. That is, the type has been [erased](https://en.wikipedia.org/wiki/Type_erasure). As such, a `dyn Trait` reference contains *two* pointers. One pointer goes to the data (e.g., an instance of a struct). Another pointer goes to a map of method call names to function pointers (known as a virtual method table or vtable).

At run-time, when a method needs to be called on the `dyn Trait`, the vtable is consulted to get the function pointer and then that function pointer is called.

See the Reference for more information on [trait objects](https://doc.rust-lang.org/reference/types/trait-object.html) and [object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety).

**Trade-offs**

The above indirection is the additional runtime cost of calling a function on a `dyn Trait`. Methods called by dynamic dispatch generally cannot be inlined by the compiler.

However, `dyn Trait` is likely to produce smaller code than `impl Trait` / generic parameters as the method won’t be duplicated for each concrete type.
</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-01-box.html)

- [dyn - Rust](https://doc.rust-lang.org/std/keyword.dyn.html)
</div></details>
<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “box 1” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/standard_library_types/`

and you can verify your work by running `rustlings run box1` command.
</blockquote></div></details>
<details>
<summary> Advanced topic </summary>
<div markdown="1">

- What is dynamic dispatch, and what are pros/cons of it compared to static dispatch?
- What are pros/cons of storing data in Heap?   
(pros/cons between `fn use_ided(x: impl Ided)` vs. `fn use_ided3(x: Box<dyn Ided>)`)
</div></details>

### Lifetime subtypes

<details>
<summary> Subtyping Explanation </summary>
<div markdown="1"> <blockquote> 

Subtyping is implicit and can occur at any stage in type checking or inference. Subtyping is restricted to two cases: variance with respect to lifetimes and between types with higher ranked lifetimes. If we were to erase lifetimes from types, then the only subtyping would be due to type equality.

Consider the following example: string literals always have `'static` lifetime. Nevertheless, we can assign `s` to `t`:

```rust
fn bar<'a>() {
    let s: &'static str = "hi";
    let t: &'a str = s;
}
```

Since `'static` outlives the lifetime parameter `'a`, `&'static str` is a subtype of `&'a str`.
</blockquote></div></details>
<details>
<summary> Lifetime bounds Explanation </summary>
<div markdown="1"> <blockquote> 

Lifetime bounds can be applied to types or to other lifetimes. The bound `'a: 'b` is usually read as `'a` *outlives* `'b`. `'a: 'b` means that `'a` lasts at least as long as `'b`, so a reference `&'a ()` is valid whenever `&'b ()` is valid.

```rust

fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
    y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
}

```

`T: 'a` means that all lifetime parameters of `T` outlive `'a`. For example, if `'a` is an unconstrained lifetime parameter, then `i32: 'static` and `&'static str: 'a` are satisfied, but `Vec<&'a ()>: 'static` is not.
</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [Understanding Rust Lifetimes](https://near.org/blog/understanding-rust-lifetimes/)

- [Subtyping and Variance](https://doc.rust-lang.org/reference/subtyping.html)

- [Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)

- [Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html?highlight=lifetimes#lifetime-bounds)
</div></details>
<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “lifetimes 1, 2, 3” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/lifetimes/`

and you can verify your work by running `rustlings run lifetimes1` command.
</blockquote></div></details>
