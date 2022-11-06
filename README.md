# Part 3

## Theme: Functional Language features

### [Part 3 Guide](https://clear-venus-f3d.notion.site/Boom-Labs-Rust-3-eecfa7e2bafe4c7a83b33104cb6a0d8b)

### 3.1. Implementing Iterator

<details>
<summary> Explanation of `iter` module </summary>
<div markdown="1"> <blockquote>

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up.

All iterators implement a trait named `Iterator` that is defined in the standard library. The definition of the trait looks like this:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

An iterator has a method, [`next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next), which when called, returns an [`Option<Item>`](https://doc.rust-lang.org/stable/std/option/enum.Option.html). Calling [`next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next) will return [`Some(Item)`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#variant.Some) as long as there are elements, and once they’ve all been exhausted, will return `None` to indicate that iteration is finished. Individual iterators may choose to resume iteration, and so calling [`next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next) again may or may not eventually start returning [`Some(Item)`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#variant.Some) again at some point (for example, see [`TryIter`](https://doc.rust-lang.org/stable/std/sync/mpsc/struct.TryIter.html)).

[`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)’s full definition includes a number of other methods as well, but they are default methods, built on top of [`next`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next), and so you get them for free.

Iterators are also composable, and it’s common to chain them together to do more complex forms of processing. See the [Adapters](https://doc.rust-lang.org/stable/std/iter/index.html#adapters) section for more details.

</blockquote></div></details>
<details>
<summary> Implement iterator module </summary>
<div markdown="1"> <blockquote>

Creating an iterator of your own involves two steps: creating a `struct` to hold the iterator’s state, and then implementing [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) for that `struct`. This is why there are so many `struct`s in this module: there is one for each iterator and iterator adapter.

Let’s make an iterator named `Counter` which counts from `1` to `5`:

```rust
// First, the struct:

/// An iterator which counts from one to five
struct Counter {
    count: usize,
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `next()`'s implementation below.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Then, we implement `Iterator` for our `Counter`:

impl Iterator for Counter {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// And now we can use it!

let mut counter = Counter::new();

assert_eq!(counter.next(), Some(1));
assert_eq!(counter.next(), Some(2));
assert_eq!(counter.next(), Some(3));
assert_eq!(counter.next(), Some(4));
assert_eq!(counter.next(), Some(5));
assert_eq!(counter.next(), None);
```

</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [Vec in std::vec - Rust](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [std::iter - Rust](https://doc.rust-lang.org/stable/std/iter/index.html)
- [Iter in std::slice - Rust](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Iterator in std::iter - Rust](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
- [The Rust Programming Language - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [The Rust Programming Language - Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
</div></details>
<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote>

Complete rustlings “iterators 1, 2, 3, 4, 5” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/standard_library_types/`

and you can verify your work by running `rustlings run iterators1` command.

</blockquote></div></details>

### Mutable Reference (&mut)

<details>
<summary> Explanation of `&mut` </summary>
<div markdown="1"> <blockquote>

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

The above code snippet causes following error: `error[E0596]: cannot borrow some_string as mutable, as it is behind a & reference`

We can fix above code to allow us to modify a borrowed value with just a few small tweaks that use, instead, a *mutable reference*:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to `s` will fail:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

</blockquote></div></details>
<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html?highlight=mutable%20reference#mutability)
</div></details>

### 3.2. Closures

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/types/closure.html)
- [Finding Closure in Rust(EN)](https://huonw.github.io/blog/2015/05/finding-closure-in-rust/)
- [Finding Closure in Rust (KR)](https://parkcheolu.tistory.com/108)
</div></details>
