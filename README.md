# Part 3

## Theme: Functional Language features

### 3.0. Start of Part 3

```bash
# git clone https://github.com/boomlabs-web3/rust-basic.git
cd rust-basic
git checkout 3.0.0
```

먼저 Part 3을 시작하기 전에, 이전 예제를 불러오겠습니다.
본 깃헙 레포를 다운받지 않으신 분들은 위 커맨드를 통해 다운로드해주세요.

<details>
<summary> main.rs </summary>
<div markdown="1"> <blockquote>

```rust
mod employee;
use employee::Employee;

mod printinfo;
use printinfo::{dynamic_print, static_print};

mod product;
use product::Product;

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);
    let product = Product::new("Apple".to_string(), 1, 100, 20220924);
    static_print(employee.clone());
    static_print(product.clone());
    dynamic_print(Box::new(employee));
    dynamic_print(Box::new(product));
}
```

</blockquote></div></details>
<details>
<summary> printinfo.rs </summary>
<div markdown="1"> <blockquote>

```rust
pub trait PrintInfo {
    fn print_info(&self);
}

pub fn static_print(data: impl PrintInfo) {
    data.print_info();
}

pub fn dynamic_print(data: Box<dyn PrintInfo>) {
    data.print_info();
}
```

</blockquote></div></details>
<details>
<summary> employee.rs </summary>
<div markdown="1"> <blockquote>

```rust
use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    pub fn new(name: String, id: u64) -> Employee {
        Employee { name, id }
    }

    // expanded or un-elided lifetime.
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(
            "employee's name: {}\nemployee's id: {}\n",
            self.name(),
            self.id()
        );
    }
}
```

</blockquote></div></details>
<details>
<summary> product.rs </summary>
<div markdown="1"> <blockquote>

```rust
use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Product {
    name: String,
    price: u64,
    quantity: u64,
    production_date: u64,
}

impl Product {
    pub fn new(name: String, price: u64, quantity: u64, production_date: u64) -> Product {
        Product {
            name,
            price,
            quantity,
            production_date,
        }
    }

    fn date(&self) -> String {
        let mut date = self.production_date;
        let year: u64 = date / 10000;
        date = date - year * 10000;
        let month = date / 100;
        date = date - month * 100;
        format!("{}/{}/{}", year, month, date)
    }
}

impl PrintInfo for Product {
    fn print_info(&self) {
        println!(
            "Product's name: {}\nProduct's price: {}\nProduct quantity: {}\nProduction date: {}",
            self.name,
            self.price,
            self.quantity,
            self.date()
        );
    }
}
```

</blockquote></div></details>

위는 Part2 마지막에 완성된 코드로, 모듈화가 되어 있습니다. 이번 파트에서는 이 중 `main.rs`와 `employee.rs`를 수정할 예정입니다.

### 3.1. Iterator

<details>
<summary> Implement `Iterator` trait in `EmployeeRecords` struct </summary>
<div markdown="1"> <blockquote>

`git checkout 3.1.0`

```rust
// employee.rs
#[derive(Debug, Clone)]
pub struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>,
}
```

먼저, `EmployeeRecords`라는 struct를 만들겠습니다.  
해당 struct 안에는 Iterator를 구현하기 위해 필요한 index field인 `idx`와 `Employee` struct들을 벡터 형태로 보관하는 `employees` field가 있습니다.

```rust
impl EmployeeRecords {
    pub fn new() -> Self {
        Self {
            idx: 0,
            employees: Vec::new(),
        }
    }

    pub fn push(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    pub fn get(&self, id: u64) -> Option<&Employee> {
        self.employees.iter().find(|e| e.id == id)
    }
}
```

그리고 위는 해당 `EmployeeRecords` struct의 메소드 구현부입니다.  
`employees` 벡터에 `employee` 를 `push`하는 `push` 메소드와, `id` 값을 입력받아, 해당 값과 `employee.id` 가 같은 `employee` 를 반환해주는 `get` 메소드입니다. 여기서, `find(|e| e.id == id)` 안에 있는 특이한 구조의 객체를 클로저(Closure)라 부르는데, 이에 대해서는 다음절에 더 자세히 다루도록 하고, 지금은 `push`와 `iter` 메소드 및 `Iterator` 트레잇에 대해 딥다이브해보고, Rust 크레이트 문서들을 읽어보는 것에 익숙해져보는 시간을 갖도록 하겠습니다.

</blockquote></div></details>

<details>
<summary> `push(&mut self, value: T)` </summary>
<div markdown="1"> <blockquote>

```rust
#[cfg(not(no_global_oom_handling))]
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn push(&mut self, value: T) {
    // This will panic or abort if we would allocate > isize::MAX bytes
    // or if the length increment would overflow for zero-sized types.
    if self.len == self.buf.capacity() {
        self.buf.reserve_for_push(self.len);
    }
    unsafe {
        let end = self.as_mut_ptr().add(self.len);
        ptr::write(end, value);
        self.len += 1;
    }
}
```

[`Vec` struct의 docs](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)를 확인해보면, 위와 같은 소스코드를 확인할 수 있습니다. 여기서 `unsafe {}` 블록 안의 내용입니다.

[`as_mut_ptr(&mut self) -> *mut T`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_ptr)은 `Vec<Employee>` 가 할당된 힙 메모리 주소를 포인팅하는 포인터를 반환해줍니다. 여기서 `.add(self.len)` 을 통해, `end` 변수는 `Vec<Employee>` 의 마지막 메모리 주소를 저장합니다.

```rust
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub const unsafe fn write<T>(dst: *mut T, src: T) {
    // We are calling the intrinsics directly to avoid function calls in the generated code
    // as `intrinsics::copy_nonoverlapping` is a wrapper function.
    extern "rust-intrinsic" {
        #[rustc_const_stable(feature = "const_intrinsic_copy", since = "1.63.0")]
        fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
    }

    // SAFETY: the caller must guarantee that `dst` is valid for writes.
    // `dst` cannot overlap `src` because the caller has mutable access
    // to `dst` while `src` is owned by this function.
    unsafe {
        copy_nonoverlapping(&src as *const T, dst, 1);
        intrinsics::forget(src);
    }
}
```

위는 [`ptr::write<T>(dst: *mut T, src: T)` 함수](https://doc.rust-lang.org/stable/std/ptr/fn.write.html)의 소스코드입니다.  
[Docs](https://doc.rust-lang.org/stable/std/ptr/fn.write.html)의 내용을 읽어보면, `write`는 `dst`에 저장된 값을 drop하지 않는다고 합니다. 또한 `src`를 drop하지 않고, `dst`가 포인팅하고 있는 메모리 주소로 이동시킵니다.

[`copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)` 함수](https://doc.rust-lang.org/std/intrinsics/fn.copy_nonoverlapping.html)는 [C의 `memcpy`](https://en.cppreference.com/w/c/string/byte/memcpy) 함수와 동등한 기능을 하는 함수입니다. 메모리 주소가 겹치지 않는 `src`와 `dst`에 한해, `src`에 저장된 값을 `dst`에 복사하는 함수입니다.

[`forget<T: ?Sized>(_: T)` 함수](https://doc.rust-lang.org/std/intrinsics/fn.forget.html)는 `src`의 값을 [drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) 없이, 스코프 밖으로 move out 시킵니다. ([forget 메소드 예제](https://doc.rust-lang.org/std/mem/fn.forget.html#examples))

위 내용들을 정리하자면, `Vec<Employee>` 메모리 주소의 마지막 위치에 `employee: Employee` 의 값을 덮어씌우고, `Vec<Employee>` 의 `len` field 값에 1을 더함으로써, 다음 `end` 값은 덮어씌운 메모리 주소 다음을 향하도록 합니다.

</blockquote></div></details>

<details>
<summary> `iter(&self) -> Iter<'_, T>` 함수와 `Iter` struct </summary>
<div markdown="1"> <blockquote>

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[inline]
pub fn iter(&self) -> Iter<'_, T> {
    Iter::new(self)
}
```

[`iter(&self) -> Iter<'_, T>` 함수](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)는 주어진 벡터 또는 슬라이스를 [`Iter` struct](https://doc.rust-lang.org/std/slice/struct.Iter.html)로 바꿔 반환해주는 함수입니다. 보다 깊은 이해를 위해 아래 `Iter` struct의 소스코드를 확인해보겠습니다.

```rust
pub struct Iter<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *const T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
    // ptr == end is a quick test for the Iterator being empty, that works
    // for both ZST and non-ZST.
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub(super) fn new(slice: &'a [T]) -> Self {
        let ptr = slice.as_ptr();
        // SAFETY: Similar to `IterMut::new`.
        unsafe {
            assume(!ptr.is_null());

            let end = if mem::size_of::<T>() == 0 {
                (ptr as *const u8).wrapping_add(slice.len()) as *const T
            } else {
                ptr.add(slice.len())
            };

            Self { ptr: NonNull::new_unchecked(ptr as *mut T), end, _marker: PhantomData }
        }
    }

	#[must_use]
    #[stable(feature = "iter_to_slice", since = "1.4.0")]
    pub fn as_slice(&self) -> &'a [T] {
        self.make_slice()
    }
}
```

`Iter` struct는 `ptr` 필드와 `end` 필드를 갖고 있습니다.  
여기서 `new` 메소드를 통해 `employees: Vec<Employee>` 벡터를 넘겨주면,  
해당 벡터의 메모리 주소를 가르키는 포인터를 `ptr`에 저장하며, `end` 에는 해당 벡터의 마지막 주소를 가르키는 포인터를 저장합니다. 여기서 `ptr == end` 라면, `Iter` struct가 비어있음을 증명합니다.

</blockquote></div></details>

<details>
<summary> 예제 코드 내 `EmployeeRecords`의 `Iterator` 구현부 </summary>
<div markdown="1"> <blockquote>

또한 [`Iter` struct는 `Iterator` 트레잇을 implement](https://doc.rust-lang.org/std/slice/struct.Iter.html#impl-Iterator-for-Iter%3C%27a%2C%20T%3E)하고 있는데요.  
`Iterator` 트레잇에 대해서는 다시 원 예제코드로 돌아와서 알아보겠습니다.

```rust
impl Iterator for EmployeeRecords {
    type Item = Employee;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            return Some(output);
        } else {
            return None;
        }
    }
}
```

[`Iterator` 트레잇 docs](https://doc.rust-lang.org/std/iter/trait.Iterator.html)를 보면, 연관 타입 `Item`과 `next` 메소드를 필수로 정의할 것을 요구합니다.

연관 타입은 “트레잇을 구현하는 단계에서 메소드에서 정의할 때, 플레이스홀더를 다른 타입으로 정의할 수 있도록 도와줍니다”. 위의 `EmployeeRecords` struct에 `Iterator` 트레잇을 구현하는 예시를 통해 좀 더 설명하겠습니다.

여기서, `type Item = Employee;` 라고 연관 타입 `Item`을 정의합니다. 이후 `next` 메서드에서와 같이 `Self::Item` 을 사용하는 부분에서 `Item` 대신 `Employee` struct로 대체합니다. 즉 위에서 “`next` 메소드는 return 값으로 `Option<Employee>` 를 반환한다”와 같은 의미입니다. (자세한 내용은 [TRPL 교과서](https://rinthel.github.io/rust-lang-book-ko/ch19-03-advanced-traits.html)를 참고해주세요.)

`next` 메소드는 `Iterator` 내에서 순회하는 규칙을 정하는 메소드입니다. `next` 메소드는 Iterator 중 하나의 항목을 `Some` 에 넣어서 반환하고, Iterator가 종료되면 `None` 을 반환합니다. 예제 코드에서 구현한 `next` 메소드에 대해 설명하자면, `EmployeeRecords` struct 안의 `idx` 필드 값과 `employees` 벡터의 길이와 비교합니다.  
`idx` 가 작으면, `employees` 벡터의 `idx` 번째 element를 `Some`에 감싸서 반환하고, `idx` 값을 1 증가시킵니다.  
만약 `idx` 필드 값이 `employees` 벡터의 길이와 같거나 크다면, `None` 을 반환합니다.

표준 라이브러리의 `Iterator` 트레잇에서 기본 구현된 다른 메소드들이 `next` 메소드를 사용하기 때문에, `next` 메소드만 정의하면 [기본 제공 메소드](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#provided-methods)를 사용할 수 있습니다.

```rust
// git checkout 3.1.1
println!("Length: {}", employee_records.clone().count());
println!("Last: {:?}", employee_records.last().unwrap());
```

예를 들면 위와 같이 `count` 메소드나 `last` 메소드를 이용할 수도 있습니다.

</blockquote></div></details>

<details>
<summary> `Iter` struct의 `Iterator` 트레잇 구현부 </summary>
<div markdown="1"> <blockquote>

다시 `Iter` struct로 돌아와서, `Iter` struct에는 `Iterator` 트레잇이 어떻게 구현되어 있는지 보겠습니다.

```rust
iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
    fn is_sorted_by<F>(self, mut compare: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>,
    {
        self.as_slice().windows(2).all(|w| {
            compare(&&w[0], &&w[1]).map(|o| o != Ordering::Greater).unwrap_or(false)
        })
    }
}}
```

구현부를 보면, `iterator!` 라는 선언적 매크로(declarative macros)를 통해 `Iterator` 트레잇을 구현하고 있습니다. 이 매크로를 조금 더 살펴보면,

```rust
macro_rules! iterator {
    (
        struct $name:ident -> $ptr:ty,
        $elem:ty,
        $raw_mut:tt,
        {$( $mut_:tt )?},
        {$($extra:tt)*}
    ) => {
        // Returns the first element and moves the start of the iterator forwards by 1.
        // Greatly improves performance compared to an inlined function. The iterator
        // must not be empty.
        macro_rules! next_unchecked {
            ($self: ident) => {& $( $mut_ )? *$self.post_inc_start(1)}
        }

        impl<'a, T> $name<'a, T> {
            // Helper function for moving the start of the iterator forwards by `offset` elements,
            // returning the old start.
            // Unsafe because the offset must not exceed `self.len()`.
            #[inline(always)]
            unsafe fn post_inc_start(&mut self, offset: isize) -> * $raw_mut T {
                if mem::size_of::<T>() == 0 {
                    zst_shrink!(self, offset);
                    self.ptr.as_ptr()
                } else {
                    let old = self.ptr.as_ptr();
                    // SAFETY: the caller guarantees that `offset` doesn't exceed `self.len()`,
                    // so this new pointer is inside `self` and thus guaranteed to be non-null.
                    self.ptr = unsafe { NonNull::new_unchecked(self.ptr.as_ptr().offset(offset)) };
                    old
                }
            }
        }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = $elem;

            #[inline]
            fn next(&mut self) -> Option<$elem> {
                // could be implemented with slices, but this avoids bounds checks

                // SAFETY: `assume` calls are safe since a slice's start pointer
                // must be non-null, and slices over non-ZSTs must also have a
                // non-null end pointer. The call to `next_unchecked!` is safe
                // since we check if the iterator is empty first.
                unsafe {
                    assume(!self.ptr.as_ptr().is_null());
                    if mem::size_of::<T>() != 0 {
                        assume(!self.end.is_null());
                    }
                    if is_empty!(self) {
                        None
                    } else {
                        Some(next_unchecked!(self))
                    }
                }
            }
        }
    }
}
```

위와 같이 복잡하게 되어있습니다. 매크로에 대한 내용은 나중에 더 다루도록 하고, 이를 조금 단순화해보자면

```rust
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T,;

    #[inline]
    fn next(&mut self) -> Option<&'a T,> {
        // could be implemented with slices, but this avoids bounds checks

        // SAFETY: `assume` calls are safe since a slice's start pointer
        // must be non-null, and slices over non-ZSTs must also have a
        // non-null end pointer. The call to `next_unchecked!` is safe
        // since we check if the iterator is empty first.
        unsafe {
            assume(!self.ptr.as_ptr().is_null());
            if mem::size_of::<T>() != 0 {
                assume(!self.end.is_null());
            }
            if is_empty!(self) {
                None
            } else {
                Some(next_unchecked!(self))
            }
        }
    }
}
```

위와 같이 `Iter` struct에도 `Iterator` 트레잇과 `next` 메소드가 구현되어 있음을 알 수 있습니다.  
해당 `next` 메소드는 포인터를 오프셋 1만큼 뒤로 이동시키는, 즉 slice의 다음 element를 가르키는 포인터를 반환시키는 메소드입니다.

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

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Iterator](https://practice.rs/functional-programing/iterator.html)
</div></details>

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
<summary> Intro of Closures </summary>
<div markdown="1"> <blockquote>

위 3.1절의 `EmployeeRecords` 구현부의 `get` 메소드를 다시 보겠습니다.

```rust
pub fn get(&self, id: u64) -> Option<&Employee> {
    self.employees.iter().find(|e| e.id == id)
}
```

보시면 `find` 메소드 안에 처음 보는 문법이 작성되어 있는 것을 확인할 수 있습니다. `find` 메소드에 대해 더 보면,

```rust
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
where
    Self: Sized,
    P: FnMut(&Self::Item) -> bool,
{
    #[inline]
    fn check<T>(mut predicate: impl FnMut(&T) -> bool) -> impl FnMut((), T) -> ControlFlow<T> {
        move |(), x| {
            if predicate(&x) { ControlFlow::Break(x) } else { ControlFlow::CONTINUE }
        }
    }

    self.try_fold((), check(predicate)).break_value()
}
```

위와 같이, `|e| e.id == id` 를 `predicate` 라는 인자로 받으며,  
`prediacate` 인자는 `FnMut(&Self::Item) -> bool` 라는 트레잇이 구현된 `P` 제네릭 타입이라고 선언하고 있습니다.

이게 바로 러스트의 클로저인데요, 이에 대해 조금 자세히 다뤄보겠습니다.

[TRPL](https://rinthel.github.io/rust-lang-book-ko/ch13-01-closures.html)에서는 *클로저*를 “변수에 저장하거나 다른 함수에 인자로 넘길 수 있는 익명 함수”라고 소개하며, “함수와 다르게 클로저는 그들이 호출되는 스코프로부터 변수들을 캡처할 수 있습니다.”라고 이야기합니다.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

위는 함수 (1번째 라인)와 클로저 (2~4 번째 라인)의 비교입니다.  
모두 같은 동작을 구현한 것이며, 모두 실제로 동작하는 코드입니다.  
마지막 라인과 같이, 클로저의 문법은 최대한으로 축약하여 사용할 수도 있습니다. 이는 클로저가 타입을 추론하는 기능을 포함하고 있기 때문입니다.

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

위와 같이 하나의 클로저에 두 가지 타입의 인자를 사용하여 호출하면 컴파일러가 에러를 뱉습니다.

TRPL의 예제를 보면서, 클로저를 사용하는 것이 유용한 상황에 대해 알아보겠습니다.

</blockquote></div></details>

<details>
<summary> 함수 #1 </summary>
<div markdown="1"> <blockquote>

`git checkout 3.2.0`

- main.rs

  ```rust
  mod function;
  use function::generate_workout;

  fn main() {
      let intensity = 5;
      let random_number = 3;

      generate_workout(intensity, random_number);
  }
  ```

- function.rs

  ```rust
  use std::thread;
  use std::time::Duration;

  fn simulated_expensive_calculation(intensity: u32) -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      intensity
  }

  pub fn generate_workout(intensity: u32, random_number: u32) {
      if intensity < 10 {
          println!(
              "Today, do {} pushups!",
              simulated_expensive_calculation(intensity)
          );
          println!(
              "Next, do {} situps!",
              simulated_expensive_calculation(intensity)
          );
      } else {
          if random_number == 0 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!(
                  "Today, run for {} minutes!",
                  simulated_expensive_calculation(intensity)
              );
          }
      }
  }
  ```

먼저 위의 함수를 이용하여 가장 기본적으로 구현한 예시를 보겠습니다.

- `simulated_expensive_calculation` 는 임의로 가정한 함수로, 2초의 계산이 걸리는 알고리즘을 가정합니다.
- `generate_workout`은 기본적인 비즈니스 로직이 구현된 함수로, 유저가 설정한 `intensity`가 10보다 작으면 위 알고리즘을 사용하여 pushup과 situp 횟수를 계산합니다.  
  `intensity`가 10보다 크면, 랜덤 넘버가 0이면 휴식 (알고리즘 사용x), 0이 아니면 역시 알고리즘을 사용하여 적절한 시간을 계산합니다.

`(intensity, random_number) = (5,3)` 인 경우, 알고리즘을 두 번 호출하게 되어 불필요하게 시간이 길어집니다. 이 부분을 한번 리팩토링하겠습니다.

</blockquote></div></details>

<details>
<summary> 함수 #2 </summary>
<div markdown="1"> <blockquote>

`git checkout 3.2.1`

- function.rs

  ```rust
  use std::thread;
  use std::time::Duration;

  fn simulated_expensive_calculation(intensity: u32) -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      intensity
  }

  pub fn generate_workout(intensity: u32, random_number: u32) {
      if intensity < 10 {
          let expensive_result = simulated_expensive_calculation(intensity);
          println!("Today, do {} pushups!", expensive_result);
          println!("Next, do {} situps!", expensive_result);
      } else {
          if random_number == 0 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              let expensive_result = simulated_expensive_calculation(intensity);
              println!("Today, run for {} minutes!", expensive_result);
          }
      }
  }
  ```

위와 같이 리팩토링 하면, `simulated_expensive_calculation` 알고리즘을 최소한으로 호출하여 시간을 줄일 수 있습니다.

다만 여기서 문제는, `simulated_expensive_calculation` 알고리즘을 호출하는 곳이 여러 군데 (2회)가 있다는 것인데요.

만약 위와 같은 알고리즘의 호출방식이 여러 번 바뀔 수 있고, 호출해야하는 곳이 지금 상황 보다 더 여러군데, 복잡하게 존재하여 업데이트하기 어려울 수 있습니다. 이러한 상황의 코드에서 `simulated_expensive_calculation` 알고리즘을 단 한군데에서만 호출하게 리팩토링해보겠습니다.

</blockquote></div></details>

<details>
<summary> 함수 #3 </summary>
<div markdown="1"> <blockquote>

`git checkout 3.2.2`

- function.rs

  ```rust
  use std::thread;
  use std::time::Duration;

  fn simulated_expensive_calculation(intensity: u32) -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      intensity
  }

  pub fn generate_workout(intensity: u32, random_number: u32) {
      let expensive_result = simulated_expensive_calculation(intensity);

      if intensity < 10 {
          println!("Today, do {} pushups!", expensive_result);
          println!("Next, do {} situps!", expensive_result);
      } else {
          if random_number == 0 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!("Today, run for {} minutes!", expensive_result);
          }
      }
  }
  ```

위와 같이 리팩토링하면, 단 한군데에서만 호출하기 때문에, `simulated_expensive_calculation` 알고리즘의 업데이트에 쉽게 대응할 수 있습니다.

이번 문제점은, `(intensity, random_number) = (10,0)` 과 같은 상황에서, 이전에는 알고리즘 호출이 없기 때문에 시간 소요가 없었다면, 이번 코드에서는 불필요하게 시간을 기다려야 한다는 점입니다. 이 문제를 클로저를 활용하여 해결해보겠습니다.

</blockquote></div></details>

<details>
<summary> 클로저 #1 </summary>
<div markdown="1"> <blockquote>

`git checkout 3.2.3`

- main.rs

  ```rust
  mod cacher;
  mod closure;
  use closure::generate_workout;

  fn main() {
      let intensity = 5;
      let random_number = 0;

      generate_workout(intensity, random_number);
  }
  ```

- closure.rs

  ```rust
  use std::thread;
  use std::time::Duration;

  pub fn generate_workout(intensity: u32, random_number: u32) {
      let expensive_closure = |num| {
          println!("calculating slowly...");
          thread::sleep(Duration::from_secs(2));
          num
      };

      if intensity < 10 {
          println!("Today, do {} pushups!", expensive_closure(intensity));
          println!("Next, do {} situps!", expensive_closure(intensity));
      } else {
          if random_number == 0 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!("Today, run for {} minutes!", expensive_closure(intensity));
          }
      }
  }
  ```

위는 **함수 #3**을 클로저를 활용해서 다시 리팩토링한 부분입니다. 알고리즘의 호출부는 `let expensive_closure` 부분 한 곳으로 통일했습니다.

다만, 여기서 `(intensity, random_number) = (5,3)` 인 경우, 알고리즘을 두 번 호출하는 **함수 #1**의 문제가 다시 생깁니다.

</blockquote></div></details>

<details>
<summary> 클로저 #2 </summary>
<div markdown="1"> <blockquote>

`git checkout 3.2.4`

- closure.rs

  ```rust
  use std::thread;
  use std::time::Duration;

  use crate::cacher::Cacher;

  pub fn generate_workout(intensity: u32, random_number: u32) {
      let mut expensive_result = Cacher::new(|num| {
          println!("calculating slowly...");
          thread::sleep(Duration::from_secs(2));
          num
      });

      if intensity < 10 {
          println!("Today, do {} pushups!", expensive_result.value(intensity));
          println!("Next, do {} situps!", expensive_result.value(intensity));
      } else {
          if random_number == 0 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!("Today, run for {} minutes!", expensive_result.value(intensity));
          }
      }
  }
  ```

- cacher.rs

  ```rust
  pub struct Cacher<T>
  where
      T: Fn(u32) -> u32,
  {
      calculation: T,
      value: Option<u32>,
  }

  impl<T> Cacher<T>
  where
      T: Fn(u32) -> u32,
  {
      pub fn new(calculation: T) -> Cacher<T> {
          Cacher {
              calculation,
              value: None,
          }
      }

      pub fn value(&mut self, arg: u32) -> u32 {
          match self.value {
              Some(v) => v,
              None => {
                  let v = (self.calculation)(arg);
                  self.value = Some(v);
                  v
              }
          }
      }
  }
  ```

예제에서는 이 문제를 `Cacher`라는 struct를 구현하여 해결했습니다.  
closure.rs의 코드를 보면, 큰 차이가 없습니다.  
`Cacher` struct는 클로저를 저장하는 `calculation` 필드와, 계산 값을 저장하는 `value` 필드로 나누어져 있습니다.  
그리고, `value ()` 메소드는 `value` 필드에 값이 저장되어 있으면 그 값을 그대로 반환하고,  
만약 저장되어 있지 않으면, 받은 인자를 `calculation`필드에 저장되어 있는 클로저에 넘겨 계산후, 값을 `value` 필드에 저장함과 동시에 반환해주는 기능입니다.

</blockquote></div></details>

<details>
<summary> 환경 캡처 </summary>
<div markdown="1"> <blockquote>

3.1절의 예시에서, `|e| e.id == id` 클로저는, 정의되었던 scope인 `get` 메소드의 `id` 변수를 ‘캡처'하여 `find` 메소드 안으로 이동합니다. 이러한 환경의 캡처는 세 가지 방식으로 가능하며, 아래 트레잇으로 표현합니다.  
아래는 TRPL의 설명을 그대로 인용한 것이며, 괄호 안은 함수에서 같은 방식으로 파라미터를 받아 처리하는 경우 (동형)의 표현입니다.

- `FnOnce (self)` 는 클로저의 *환경*으로 알고 있는, 그것을 둘러싼 환경에서 캡처한 변수 들을 소비합니다. 캡처한 변수를 소비하기 위해, 클로저는 이 변수의 소유권을 가져야 하고 그것이 정의될 때 클로저 안으로 그것들을 옮겨와야 합니다. 이름의 일부인 `Once` 는 그 클로저가 동일한 변수들에 대해 한번이상 소유권을 얻을수 없다는 사실을 의미하며, 그래서 한 번만 호출 될 수 있습니다.
- `Fn (&self)` 은 그 환경으로 부터 값들을 불변으로 빌려 옵니다.
- `FnMut (&mut self)` 값들을 가변으로 빌려오기 때문에 그 환경을 변경할 수 있습니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/types/closure.html)
- [Finding Closure in Rust(EN)](https://huonw.github.io/blog/2015/05/finding-closure-in-rust/)
- [Finding Closure in Rust (KR)](https://parkcheolu.tistory.com/108)
</div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Closure](https://practice.rs/functional-programing/closure.html)
</div></details>
