# Part 4

## Theme: Unsafe Rust

### 4.0. Start of Part 4

이번 주에는 Part 3의 내용을 wrap up하고, 그 과정에서 Life time, ownership, drop, destructor, 및 unsafe의 일부에 대해 더 자세히 다뤄보도록 하겠습니다.

Part 3에서 다루었던 내용인 `mem::forget` 을 `ptr::write` 메소드에서 사용하는 의의에 대해 이야기해보면서, 자연스럽게 주제를 풀어나가보겠습니다.

### 4.1. Ownership, Borrowing, and Lifetime

<details>
<summary> Explanation of Rustviz </summary>
<div markdown="1"> <blockquote>

```bash
cargo install mdbook
git clone https://github.com/boomlabs-web3/rustviz.git
cd rustviz/rustviz_mdbook
./view_examples.sh
mdbook serve -p 8000
```

위 커맨드를 입력하여, rustviz 설치 및 localhost로 웹페이지를 배포해주세요.

[http://localhost:8000/](http://localhost:8000/)  
그리고 위 링크를 클릭하여 배포된 페이지를 통해 시각화된 ownership, borrowing 그래프를 보며 공부하시면 됩니다.  
이번 세션에서는 그 중 아래 주제들에 대해 다루겠습니다.

- copy
- move_assignment
- func_take_ownership
- immutable_borrow
- mutable_borrow
- nll_lexical_scope_different
- struct_lifetime
- immutable_borrow_lifetime

</blockquote></div></details>

<details>
<summary> Explanation of Lifetime </summary>
<div markdown="1"> <blockquote>

```rust
fn longest(x: &String, y: &String) -> &String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}
```

라이프타임에 대해 설명하기 위해, 1주차의 예제로 다시 돌아오겠습니다.

<img width="949" alt="Untitled" src="https://user-images.githubusercontent.com/96561121/201529120-6c1963e8-4e9f-48f3-8dbf-7f88df90b201.png">

아시다시피, 위 예제는 [E0106](https://doc.rust-lang.org/error_codes/E0106.html) 컴파일 에러를 뱉습니다.

함수의 output을 `result` 에 할당할 때에, 어떤 값을 참조할 지 borrow checker가 판단할 수 없다는 이야기입니다.

```rust
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}
```

이는 위와 같이 라이프타임의 선언부에서 명시하여, 해결할 수 있습니다.

> 구체적인 참조자들이 `longest`로 넘겨질 때, `'a`에 대입되게 되는 구체적인 라이프타임은 `y`의 스코프와 겹치는 `x` 스코프의 부분입니다. 스코프는 언제나 중첩되기 때문에, 이것이 제네릭 라이프타임 `'a`이다라고 말하는 또 다른 방법은 `x`와 `y`의 라이프타임 중에서 더 작은 쪽과 동일한 구체적인 라이프타임을 구하는 것일 겁니다. 반환되는 참조자에 대해서도 같은 라이프타임 파라미터인 `'a`를 명시했으므로, 반환되는 참조자도 `x` 와 `y`의 라이프타임 중 짧은 쪽만큼은 길게 유효함을 보장할 것입니다.

이를 TRPL에서는 위와 같이 서술하고 있는데요. 이를 조금 더 알기 쉽게 설명하자면, 함수 `longest`로부터 return값을 할당받은 `result`의 라이프타임보다, `string1`과 `string2`의 라이프타임이 항상 같거나 더 길도록 보장한다는 이야기입니다.

```rust
fn longest<'a, 'b: 'a>(x: &'a String, y: &'b String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}
```

또는 위와 같이 lifetime subtyping을 통해 함수의 input 중에 어떤 값을 함수의 output이 라이프타임을 참조할 지 선택할 수 있습니다. 위 예시에서, `result`는 `string1`의 라이프타임과 같은 scope의 라이프타임을 가지며, `string2`의 라이프타임은 최소한 `result`의 라이프타임보다 더 길도록 보장합니다.

이를 통해 `result`를 사용하는 라인 (예: `println!("The longest string is {}", result);`)에서 참조하는 값이 `result`보다 먼저 `drop`(정확히는 scope에서 move out)되는 일이 없도록 보장합니다. 만일 먼저 `drop`된다면 컴파일 타임에서 에러가 생깁니다. 아래는 그 예시입니다.

```rust
use std::mem;

fn longest<'a, 'b: 'a>(x: &'a String, y: &'b String) -> &'a String {
    x
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    mem::forget(string2);
    println!("The longest string is {}", result);
}
```

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [Rustviz](https://github.com/rustviz/rustviz)
- [The Rust Programming Language - Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote>

Complete rustlings “lifetimes 1, 2, 3” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/lifetimes/`

and you can verify your work by running `rustlings run lifetimes1` command.

</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Lifetime 17.1 - 17.3](https://practice.rs/lifetime/intro.html)
</div></details>

### 4.2. Wrap-up of Part 3

<details>
<summary> Explanation of `Vec` struct </summary>
<div markdown="1"> <blockquote>

`Vec`은 (pointer, len, capacity) 삼중항으로 항상 구성되어 있습니다.

포인터가 실제로 할당된 메모리를 가르치지 않을 수도 있는데,  
[`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new), [`vec![]`](https://doc.rust-lang.org/std/macro.vec.html), [`Vec::with_capacity(0)`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity) 등을 통해 capacity가 0인 `Vec`를 생성하면 메모리가 할당되지 않습니다. 또한 `Vec` 내에 사이즈가 0인 타입을 저장하면 역시 메모리가 할당되지 않습니다.  
즉, [`mem::size_of::<T>`](https://doc.rust-lang.org/std/mem/fn.size_of.html)`() * `[`capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity)`() > 0` 인 경우 (⇔ 타입의 사이즈와 벡터의 capacity가 모두 0보다 클 때)에 대해 메모리가 할당됩니다 (필요충분조건).

만약 할당된 메모리를 가리킨다면, `ptr`는 힙 메모리의 주소를 가르키고, 해당 메모리 주소부터 `len` 개의 인수들이 연속되어 저장되고, 그 다음엔 `capacity - len` 개의 uninitialized된 인수들이 연속되어 저장됩니다.

```rust
            ptr      len  capacity
       +--------+--------+--------+
       | 0x0123 |      2 |      4 |
       +--------+--------+--------+
            |
            v
Heap   +--------+--------+--------+--------+
       |    'a' |    'b' | uninit | uninit |
       +--------+--------+--------+--------+
```

예를 들면 위 예시는 capacity 4인 벡터가 ‘a’와 ‘b’ element를 포함하고 있는 것을 보여줍니다. 상단은 `Vec` 구조로, 할당된 힙 메모리의 첫 번째를 포인팅하는 포인터와, 길이, capacity를 포함합니다. 아랫 부분은 할당된 힙 메모리입니다.

`Vec` struct는 아래와 같은 특징이 있습니다.

- small optimization (SSO of c++)을 절대 수행하지 않습니다.
- `Vec`의 요소가 전부 비워지더라도, 자동적으로 shrink - 메모리의 일부 할당 해제 -가 발생하지 않습니다. 즉, `Vec` 을 전부 비운 후에, 다른 요소로 채우더라도, allocator가 호출되지 않습니다. manually하게 [`shrink_to_fit`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit) 또는 [`shrink_to`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to)를 호출해야 합니다.
- `len == capacity`가 되지 않는 한, `push`나 `insert`가 재할당을 하지 않습니다.

</blockquote></div></details>

<details>
<summary> Explanation of `mem::forget` (unsafe rust) </summary>
<div markdown="1"><blockquote>

```rust
fn main() {
    use std::mem;

    let mut v = vec![65, 122];
    // `v`의 내용으로 Sting `s`를 생성
    let s = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };
    println!("{:?}", v);

	// `v`의 메모리는 `s`에 의해 관리되기 때문에 leak.
    mem::forget(v);      // 이 부분을 주석처리하면 double-free error가 생김.
    assert_eq!(s, "Az");
    mem::drop(s);        // `s` 는 drop되고, 메모리는 dealloc됨.
}
```

Rust의 safe 코드는 대부분의 상황에서 메모리 leak에 대하여 “안전”하고 엄격하게 관리한다는 가정하는 것이 합리적이고, 대부분의 상황에서는 맞아 떨어집니다. 하지만, 메모리 leak에 대해 safe하지 않은 상황은 생길 수 있습니다. 가장 대표적인 예가 [순환 참조하는 참조자를 만들어 메모리가 `drop`되지 않는 경우](https://rinthel.github.io/rust-lang-book-ko/ch15-06-reference-cycles.html)입니다.

위의 상황을 인정하게 되면서, `mem::forget`의 사용을 완전히 배제하던 분위기가 누그러졌습니다. `mem::forget`은 destructor를 호출하지 않고, value를 consume하기 때문에, manually 메모리 leak을 야기할 수도 있습니다. 다만 `unsafe` 코드에서 주로 두 가지 경우에서 사용됩니다.

1. 같은 메모리 주소를 포인팅하는 2개 이상의 포인터가 있을 경우 - double-free를 방지하기 위해 사용합니다.
2. uninitialized 메모리를 참조하는 포인터를 다룰 경우 - uninitialized memory를 destruct하는 것은 큰 버그를 야기할 수 있습니다.

```rust
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
#[cfg_attr(miri, track_caller)]
pub const unsafe fn write<T>(dst: *mut T, src: T) {
    extern "rust-intrinsic" {
        #[rustc_const_stable(feature = "const_intrinsic_copy", since = "1.63.0")]
        fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
    }
    unsafe {
        copy_nonoverlapping(&src as *const T, dst, 1);  // src에 대한 새로운 raw pointer 생성
        intrinsics::forget(src);                        // 여기서 forget하지 않으면 double-free
    }
}
```

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [Vec in std::vec - Rust](https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/leaking.html?highlight=forget#leaking)
- [Does std::ptr::write transfer the "uninitialized-ness" of the bytes it writes?](https://stackoverflow.com/questions/61114026/does-stdptrwrite-transfer-the-uninitialized-ness-of-the-bytes-it-writes)
</div></details>

### 4.3. Unsafe Rust

<details>
<summary> Explanation of Unsafe Rust </summary>
<div markdown="1"> <blockquote>

`unsafe` Rust는 블록 내에서 아래와 같은 기능을 가능케합니다.

- 로우 포인터 (raw pointer) 를 역참조하기
- `unsafe` 함수 혹은 메소드 호출하기
- 가변 정적 변수 (mutable static variable) 의 접근 혹은 수정하기
- `unsafe` 트레잇 구현하기
- `union` 타입의 필드에 접근하기

여기서 알아야 하는 것은 위 5가지 기능 외에 다른 기능을 제공하지 않는다는 것입니다. 즉, `unsafe` 키워드는 borrow checker를 무시하거나, Rust의 다른 safety check를 피하는 기능을 제공하지 않습니다.

위와 같은 동작이 `unsafe` 블록 안으로만 제한되는 이유는 Undefined Behavior (UB)가 야기되는 것을 막기 위함입니다.

UB들의 종류는 아래와 같습니다.

- 댕글링 포인터의 역참조
- [pointer aliasing rules](https://doc.rust-lang.org/nomicon/references.html) 위반  
  예: 참조 대상이 드랍된 scope에서 참조자 이용
- 잘못된 ABI를 통한 함수 호출 / 잘못된 unwind ABI를 통한 함수 unwinding
- [data race](https://doc.rust-lang.org/nomicon/races.html) 발생
- 현재 스레드가 지원하지 않는 [target features](https://doc.rust-lang.org/reference/attributes/codegen.html#the-target_feature-attribute) 로 컴파일된 코드 실행  
  예: aarch_64 환경에서 x86_64(인텔칩) 코드 실행
- 타입에 맞지 않은 잘못된 값 생성
  - a `bool` that isn't 0 or 1
  - an `enum` with an invalid discriminant
  - a null `fn` pointer
  - a `char` outside the ranges [0x0, 0xD7FF] and [0xE000, 0x10FFFF]
  - a `!` (all values are invalid for this type)
  - an integer (`i*`/`u*`), floating point value (`f*`), or raw pointer read from [uninitialized memory](https://doc.rust-lang.org/nomicon/uninitialized.html), or uninitialized memory in a `str`.
  - a reference/`Box` that is dangling, unaligned, or points to an invalid value.
  - a wide reference, `Box`, or raw pointer that has invalid metadata:
    - `dyn Trait` metadata is invalid if it is not a pointer to a vtable for `Trait` that matches the actual dynamic trait the pointer or reference points to
    - slice metadata is invalid if the length is not a valid `usize` (i.e., it must not be read from uninitialized memory)
  - a type with custom invalid values that is one of those values, such as a [`NonNull`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) that is null. (Requesting custom invalid values is an unstable feature, but some stable libstd types, like `NonNull`, make use of it.)

위 다섯 가지 기능에 대해서 나머지 넷에 대해서는 어느정도 익숙하실 거라 생각합니다. 나머지 5번째 기능에 대해 알아보겠습니다.

</blockquote></div></details>

<details>
<summary> Explanation of `union` type </summary>
<div markdown="1"> <blockquote>

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f64,
}
```

`union`이란 위와 같은 방식으로 선언할 수 있습니다. `struct` 의 선언과 상당히 유사합니다. 다만 `struct` 대비 `union`의 가장 큰 차이점은, `union`은 모든 필드의 값들이 같은 storage를 공유한다는 것입니다.

따라서 위 예시에서, `f1` 필드에 데이터를 overwrite하는 것은, `f2` 필드에도 영향을 주며, 또한 `union`의 사이즈는 필드 중에서 사이즈가 가장 큰 필드에 맞춰 결정됩니다. (예시에선 `f64`)

`union` 필드의 타입들은 아래 타입들로 제한됩니다.

- `Copy` types
- References (`&T` and `&mut T` for arbitrary `T`)
- [`ManuallyDrop<T>` (for arbitrary `T`)](https://doc.rust-lang.org/stable/std/mem/struct.ManuallyDrop.html)
- Tuples and arrays containing only allowed union field types

여기서 `ManuallyDrop<T>`는 wrapper struct로, 감싼 타입이 scope 밖으로 벗어나더라도 destructor가 자동적으로 호출되지 않도록 억제합니다.

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: u64,
}

fn main() {
    let u = MyUnion { f1: 1 };
    println!("{}, {}", unsafe { u.f1 }, unsafe { u.f2 });
}
```

위는 간단한 예제인데요. 보시면 `u.f1`값과 `u.f2`값이 같은 메모리를 공유함을 알 수 있습니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://rinthel.github.io/rust-lang-book-ko/ch19-01-unsafe-rust.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/what-unsafe-does.html)
- [Unsafe Rust: How and when (not) to use it - LogRocket Blog](https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/)
- [The Rust Reference](https://doc.rust-lang.org/reference/items/unions.html)
- [The Rust RFC Book - 1444](https://rust-lang.github.io/rfcs/1444-union.html)
- [Is it a good practice to use unions in C++?](https://stackoverflow.com/questions/943267/is-it-a-good-practice-to-use-unions-in-c)
- [Purpose of Unions in C and C++](https://stackoverflow.com/questions/2310483/purpose-of-unions-in-c-and-c)
- [Why do we need C Unions?](https://stackoverflow.com/questions/252552/why-do-we-need-c-unions)
</div></details>

### 4.4. Purpose of Phantom Data

<details>
<summary> Explanation of `PhantomData` </summary>
<div markdown="1"> <blockquote>

When working with unsafe code, we can often end up in a situation where types or lifetimes are logically associated with a struct, but not actually part of a field. This most commonly occurs with lifetimes.

For example, when using raw pointers one may wish to specify the lifetime for which the pointed-at data is valid. An initial attempt (below) causes this error:

```rust
struct AtomicPtr<T> { // unused type parameter T
    data: AtomicUint  // represents an atomically mutable *mut T, really
}

struct Foo<'a, T> {
    x: *const T,
}
```

Since these parameters are unused, the inference can reasonably conclude that `AtomicPtr<int>` and `AtomicPtr<uint>` are interchangeable: after all, there are no fields of type `T`, so what difference does it make what value it has? This is not good (and in fact we have behavior like this today for lifetimes, which is a common source of error).

To avoid this hazard, the RFC proposes to make it an error to have a type or lifetime parameter whose variance is not constrained. Almost always, the correct thing to do in such a case is to either remove the parameter in question or insert a *marker type*. Marker types basically inform the inference engine to pretend as if the type parameter were used in particular ways.

We want to express the constraint that Foo should not outlive `'a`, because the data pointed to by `T` is only valid for that lifetime. The problem is that there are no actual uses of `'a`. It's possible to work around this by adding a PhantomData type to the struct, using it to tell the compiler to act as if the struct contained a borrowed reference `&'a T`:

```rust
use std::marker::PhantomData;

struct Foo<'a, T: 'a> {
    x: *const T,
    phantom: PhantomData<&'a T>
}

struct AtomicPtr<T> {
    data: AtomicUint,

    // Act as if we could reach a `*mut T` for variance. This will
    // make `AtomicPtr` *invariant* with respect to `T` (because `T` appears
    // underneath the `mut` qualifier).
    marker: PhantomData<*mut T>,
}

pub struct Items<'a, T: 'a> {
    ptr: *const T,
    end: *const T,

    // Act as if we could reach a slice `[T]` with lifetime `'a`.
    // Induces covariance on `T` and suitable variance on `'a`
    // (covariance using the definition from rfcs#391).
    marker: marker::PhantomData<&'a [T]>,
}
```

An instance of `PhantomData` is used to represent data that is logically present, although the type system cannot see it. `PhantomData` is covariant with respect to its type parameter `T`.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/phantom-data.html)
- [What is the `PhantomData` actually doing in the implementation of `Vec`?](https://stackoverflow.com/questions/41533508/what-is-the-phantomdata-actually-doing-in-the-implementation-of-vec/42720413)
- [Error codes index](https://doc.rust-lang.org/error_codes/E0392.html)
- [The Rust RFC Book](https://rust-lang.github.io/rfcs/0738-variance.html#the-corner-case-unused-parameters-and-parameters-that-are-only-used-unsafely)
</div></details>
