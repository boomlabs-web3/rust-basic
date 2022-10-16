# Part 1

## Theme: Basic Syntax

### 1.0. Hello world

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

```bash
cargo new rust-basic
```

위 명령어를 커맨드라인에 입력하면 Cargo가 rust-basic이라는 기본적인 Rust Scaffold 패키지를 생성합니다.

여기서 Cargo는 Rust의 패키지 매니저이자, 컴파일, 유닛 테스트 등의 기능을 제공해주는 아주 유용한 공식 어플리케이션입니다. Cargo의 기능에 대해 더 자세히 알고싶으신 분은 “[The Cargo Book](https://doc.rust-lang.org/cargo/)”을 참고해주세요.

```
rust-basic
├─ src
│  └─ main.rs
├─ target
│  └─ debug
├─ Cargo.toml
└─ Cargo.lock
```

새 패키지를 생성하면, 위와 같은 계층구조로 패키지가 생성됨을 확인할 수 있습니다.

아직 실행파일을 build하지 않았더라면, Cargo.lock과 target 폴더는 생성되지 않았을 것입니다. 여기서 위 구성요소 각각에 대해 가볍게 설명하겠습니다.

- src 폴더는 *.rs로 끝나는 소스 코드들을 보관하는 폴더입니다.
- target 폴더는 실행 파일을 빌드하면 그 결과가 저장되는 폴더입니다.
- Cargo.toml은 개발자가 디펜던시 등 패키지 정보에 대해 직접 작성하는 곳입니다.
    
    ```toml
    [package]
    name = "rust-basic"
    version = "0.1.0"
    edition = "2021"
    
    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    
    [dependencies]
    ```
    
    - [package] 밑에는 패키지 이름, 패키지의 버전, author 등이 있으며, edition은 사용하는 rust 언어의 버전을 의미합니다.
    - [dependencies]는 패키지가 사용하는 다른 라이브러리(crate)들의 이름과 버전을 적는 곳입니다.
    - 그 외 나머지는 개발 과정에서 차차 알게 될 것입니다.
- Cargo.lock은 Cargo.toml과 그 기능이 유사하나, 개발자가 manual하게 작성하는 것이 아닌 rust 컴파일러가 빌드 과정에서 자동적으로 생성되는 파일입니다. (더 자세한 내용은 다음 글들을 참고해주세요: [Cargo.toml vs Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html), [Cargo.lock의 의의](https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries))

    ![Untitled](https://user-images.githubusercontent.com/96561121/201534016-6d94d0f2-1c12-44f5-9ebc-f1ee0bb25f04.png)

rust-basic 폴더에서 `cargo run` 명령어를 입력하면 실행파일이 빌드되고, 위와 같이 “Hello, world!”가 출력됩니다.

```rust
fn main() {
    println!("Hello, world!");
}
```

src/main.rs 파일을 확인하면, `main()` 함수 안에 `println!` 매크로가 작성되어있는 것을 확인할 수 있습니다.  
이를 [Declarative macro](https://rinthel.github.io/rust-lang-book-ko/appendix-04-macros.html#%EC%9D%BC%EB%B0%98%EC%A0%81%EC%9D%B8-%EB%A9%94%ED%83%80%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%98%EB%B0%8D%EC%9D%84-%EC%9C%84%ED%95%9C-macro_rules-%EB%A5%BC-%EC%82%AC%EC%9A%A9%ED%95%98%EB%8A%94-%EC%84%A0%EC%96%B8%EC%A0%81-%EB%A7%A4%ED%81%AC%EB%A1%9C)라고 부르는데, 자세한 것은 나중에 다시 이야기하겠습니다.  
지금은 `println!` 매크로가 안에 있는 문자열을 터미널에 출력한다 정도로만 이해하면 되겠습니다.

</blockquote></div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"><blockquote> 

Complete rustlings “intro1, 2” exercises.

Boilerplate codes are stored in path below,  
`rustlings/exercises/intro/`

and you can verify your work by running `rustlings run intro1` command.
</blockquote></div></details>


### 1.1. Testing

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

다음으로는 Rust의 unit test에 대해 알아보겠습니다.

```bash
git clone https://github.com/boomlabs-web3/rust-basic.git
cd rust-basic
git checkout 1.1.0
```

**“1.0. Hello world”** 절에서 생성한 패키지에 아래 코드를 붙여도 좋고,  
또는 위 명령어를 입력하여 예제 코드를 받은 후에, 1.1.0 태그로 체크아웃하셔도 좋습니다.

```rust
// rust-basic/src/main.rs
#[test]
fn my_test() {
    assert!(true);
}

fn main() {
    println!("Hello, world!");
}
```

위와 같이 main함수 위에 테스트 코드를 작성해보겠습니다.

![Untitled 1](https://user-images.githubusercontent.com/96561121/201534051-cc0d7e6f-3612-46f0-b23a-7fff40f0f408.png)

그 후, `cargo test`를 터미널에 입력하면, 위와 같이 unit test가 한 개 통과하였다고 출력됩니다.

![Untitled 2](https://user-images.githubusercontent.com/96561121/201534066-a3a9f6ad-4066-4ad7-8161-db9a6f11ac6c.png)

참고로 VScode에 extension을 설치하여 사용하시면, 소스 코드에서 버튼을 눌러 테스트, 디버깅, 실행 등을 할 수도 있습니다. 

다시 소스 코드로 돌아가서, 

- `#[test]`는 procedural macro 중 [attribute macro](https://veykril.github.io/tlborm/proc-macros/methodical/attr.html)라 불리우는 것입니다.  
자세한 설명은 나중으로 넘기고, 이 macro가 하는 일은 컴파일러로 하여금 밑에 있는 함수(여기서는 `fn my_test()`)를 테스트 코드로 취급하도록 합니다.  
즉, `cargo run` 을 통해 실제 실행 파일을 빌드하고 실행할 때에는 무시하지만, `cargo test`를 통해 유닛테스트를 돌릴 때에는 위 테스트 코드를 돌려 테스트의 통과 여부를 출력해줍니다.  
- `assert!(true);`에서 `assert!`는 앞서 보았던 `println!`처럼 declarative macro 입니다.  
괄호 안에는 참 또는 거짓을 반환하는 expression이 들어가는데, 참인지를 assertion하는 기능입니다.  
테스트 코드 안의 `assert!` 매크로가 거짓을 뱉으면, 테스트는 실패합니다.
    
    ```rust
    #[test]
    fn my_test() {
    	let x = 1;
        assert!(x == 2);
    }
    ```
    
    위와 같이 false를 뱉는 expression을 넣고 테스트를 돌리면, cargo는 아래와 같이 test가 실패했고, 어디에서 실패했는지를 알려줍니다.
    
    ![Untitled 3](https://user-images.githubusercontent.com/96561121/201534083-ae18bf06-46b3-4e3c-a6da-54f93ece9b1f.png)

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1"> 

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/attributes/testing.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"><blockquote> 

Complete rustlings “tests1, 2, 3” exercises.

Boilerplate codes are stored in path below,  
`rustlings/exercises/tests/`

and you can verify your work by running `rustlings run tests1` command.
</blockquote></div></details>

### 1.2. Functions

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

다음은 Rust의 함수입니다.

```rust
// git checkout 1.2.0
// rust-basic/src/main.rs
fn greet(x: u64) {
    println!("Hello to number {}", x);
}

fn main() {
    greet(100);
}
```

위는 가장 기본적인 함수의 구성입니다. 이를 뜯어보면,  
`fn greet()`이라는 이름의 함수는 `x`라는 인자를 받습니다. 이 때, `x: u64`와 같이, 받는 인자의 타입을 명시해줘야 합니다.

`main`함수 안에서는 `greet(100)`과 같이 uint 64 타입의 100 값을 인자로 넘겨주어 `greet` 함수를 호출해줍니다.

이 함수는 아무런 값을 반환하지 않지만, 아래와 같이 반환하게 코드를 짤 수도 있습니다.

```rust
fn greet(x: u64) -> u64 {
    println!("Hello to number {}", x);
    return x;
}

fn main() {
    let a = greet(100);
    println!("{}", a);
}
```

만약 함수가 값을 반환한다면, `-> u64` 처럼, 반환하는 값의 타입도 명시해야 합니다.

```rust
fn greet(x: u64) -> u64 {
    println!("Hello to number {}", x);
    x
}
```

참고로 Rust에서는 위와 같이 return 문을 생략할 수도 있습니다.  
주의해야 할 점은 return문을 생략하려면, expression만 입력해야 합니다.

더 자세히 이야기하면, `return (expression);` 이것은 어떤 값(expression)을 반환하는 것을 실행하는 것이기 때문에 statement입니다. 따라서 ;(세미콜론)으로 끝맺음을 해야 합니다.  
만약, return문을 생략하고자 하면, 반환하고자 하는 expression만 입력해야 하기 때문에 ;(세미콜론)을 생략해야 합니다.

```rust
fn greet(x: u64) -> u64 {
    println!("Hello to number {}", x);
    x;
}
```

위와 같이 ;을 붙이면, statement은 아무런 값을 반환하지 않기 때문에 예상했던 return type (u64)과 실제 return되는 type이 다르다는 에러를 컴파일러가 뱉게 될 것입니다.  
return 문을 생략하여 사용한 코드도 많이 마주치게 될 것이므로, 익숙해지는 것이 좋습니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"><blockquote>

Complete rustlings “variables 1, 2, 3, 4, 5, 6” exercises.

Boilerplate codes are stored in path below,  
`rustlings/exercises/variables/`

and you can verify your work by running `rustlings run variables1` command.
</blockquote></div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"><blockquote> 

Complete rustlings “functions 1, 2, 3, 4, 5” exercises.

Boilerplate codes are stored in path below,  
`rustlings/exercises/fuctions/`

and you can verify your work by running `rustlings run functions1` command.
</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Functions](https://practice.rs/basic-types/functions.html)
</div></details>

### 1.3. Strings

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

다음은 Rust의 `String` 타입에 대해 이야기해보겠습니다.

```rust
// git checkout 1.3.2
// rust-basic/src/main.rs
fn greet(x: String) {
    println!("Hello to {}", x);
}

fn main() {
    let greeting: &str = "Hello, world!";
    let my_greeting: String = "Hello, world!".to_string();
    greet(my_greeting);
}
```

Rust에는 문자열을 처리하는 타입이 `&str`과 `String`이 있습니다. 이 둘의 차이에 대해서는 아래 Reference의 문서들을 정독해주시길 바랍니다.
간략하게 요약하자면 아래와 같습니다.

- `&str`: 스트링 슬라이스라고도 불리우는데, 스택 또는 힙 메모리 어딘가에 저장되어 있는 값을 pointing합니다. 여기서 `&`기호는 참조자를 의미하는데, 이에 대해서는 뒤에서 더 자세히 설명하겠습니다.  
`&str`은 컴파일 타임에 값이 결정되는 값으로, 정적인 값입니다. 즉, 런타임에서 값이 바뀌지 않으며, binary executable에 하드코딩됩니다.
- `String`: 힙 메모리에 저장되는 값이며, vector type과 유사합니다.
    
    ```rust
    #[derive(PartialOrd, Eq, Ord)]
    #[cfg_attr(not(test), rustc_diagnostic_item = "String")]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub struct String {
        vec: Vec<u8>,
    }
    ```
    
    실제 std library의 소스 코드를 참고하면 위와 같습니다.  
    `String`은 런타임에서 문자열의 크기가 동적으로 변할 수 있는 상황 - 예를 들면, CLI 어플리케이션에서 argument로 문자열을 받는 상황 - 을 위해 만들어진 타입입니다.  
    즉 이를 위해 얼마나 많은 메모리를 할당해주어야 하는지 컴파일러는 모르기 때문에, `String`은 힙메모리 저장이 강제됩니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html?highlight=String#the-string-type)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-02-strings.html#what-is-a-string)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “strings 1, 2, 3, 4” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/strings/`

and you can verify your work by running `rustlings run strings1` command.
</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [String](https://practice.rs/collections/string.html)
</div></details>

### 1.4. Structs

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

Rust의 Struct(구조체)는 다른 언어의 Class와 유사한 개념입니다.

```rust
// git checkout 1.4.0
// rust-basic/src/main.rs
struct Employee {
    name: String,
    id: u64,
}

fn main() {
    let employee = Employee {
        name: "John".to_string(),
        id: 101,
    };
    println!("{}", employee.name);
}
```

먼저, Rust의 구조체는 `struct` 키워드를 사용하여 선언합니다. 위 코드블록에서 `struct Employee`입니다.  
그 아래에는 구조체의 각 구성요소들(field)을 명명하고, 각 구성요소의 데이터 타입도 선언합니다.

이후 선언한 구조체를 사용하기 위해, 각 field의 값을 명시한 인스턴스를 생성합니다. 위 코드블록에서는 `employee`입니다.  
* 여기서, Rust의 일반적인 코딩 컨벤션은 struct, enum, trait 등의 구조 타입들을 명명할 때에는 CamelCase를 사용하고, 변수나 해당 구조 타입의 인스턴스를 명명할 때에는 snake_case를 사용합니다. (관련 내용은 [API 가이드라인](https://rust-lang.github.io/api-guidelines/naming.html)을 참고해주세요.)

구조체 인스턴스 내부의 값에 접근을 하려면, `employee.name`과 같이 점 표기법을 이용하여 접근합니다. 

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/items/structs.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “structs 1, 2” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/structs/`

and you can verify your work by running `rustlings run structs1` command.
</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Struct](https://practice.rs/compound-types/struct.html)
</div></details>

### 1.5. Enums

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

```rust
// git checkout 1.5.0
// rust-basic/src/main.rs
enum IsTrue {
    True(u64),
    False,
}

fn main() {
    let my_value = IsTrue::True(100);

    match my_value {
        IsTrue::True(x) => println!("True {}", x),
        IsTrue::False => println!("False"),
    }
}
```

Enum 타입은 enumeration의 준말입니다. 다른 프로그래밍 언어를 사용하시던 분이라면 익숙하실 수도 있을텐데요.

Rust의 enum 타입은 내부의 field 값의 타입을 여러가지 중 하나로 강제해야 하는 경우에 사용됩니다.  
예를 들면 위 예제에서 `my_value`는 `IsTrue::True(u64)` 또는 `IsTrue::False` 중 하나의 타입만 가질 수 있는 것이죠.

또한 타입 내부에 다른 데이터 타입을 실을 수도 있습니다. 여기에는 `u64`, `bool` 등과 같은 기본 데이터 타입 뿐만 아니라, `String`, `Vec` 등과 같은 `struct` 타입, `enum` 타입도 넣을 수 있습니다.

Enum 타입은 [`match`](https://rinthel.github.io/rust-lang-book-ko/ch06-02-match.html) 연산자와 [`if let`](https://rinthel.github.io/rust-lang-book-ko/ch06-03-if-let.html) 연산자를 이용하여 다룰 수 있습니다. 각 연산자의 특성은 링크로 걸어둔 Rust 교과서를 참고해주시길 바랍니다. 간단하게 `match`의 경우엔 보통 enum 타입으로 만들어지는 여러 갈래의 분기점을 모두 다뤄야할 때 사용됩니다. `if let` 문법의 경우에는 enum 타입으로 만들어지는 여러 갈래 중, 한 갈래를 더 특정지어서 다루고 나머지는 무시해도 될 때, 코드를 더 간결하게 작성하고 싶을 때 이용합니다.

위 예시에서는 `match`를 이용하여 다루고 있습니다.  
`IsTrue::True(x) => println!("True {}", x),` 는 `my_value`의 enum 타입이 `IsTrue::True(_)`일 때, `True()`안에 있는 값을 꺼내어 출력하겠다는 의미입니다. 여기서 underscore(`_`)는 rust에서 어떤 값이 와도 상관없이, 그 값을 무시하고 처리하고 싶을 때 사용합니다.

`IsTrue::True(100) => println!("True {}", x),`와 같이, `my_value` 의 `True()`가 싣고 있는 특정 데이터 만을 다룰 수도 있습니다.

```rust
let my_value = IsTrue::True(100);

match my_value {
    IsTrue::True(50) => println!("True 50"),
    IsTrue::True(_) => println!("Not True"),
    IsTrue::False => println!("False"),
}
```

예를 들면 위와 같이 코드를 짜면, “Not True”가 출력됩니다.

```rust
let my_value = IsTrue::True(100);

match my_value {
    IsTrue::True(50) => println!("True 50"),
    IsTrue::False => println!("False"),
    _ => println!("Not True"),
}
```

또는 위와 같이 다룰 수도 있습니다.

여기서 match 연산자는 위에서 부터 실행을 합니다. 그 말은,

```rust
let my_value = IsTrue::False;

match my_value {
    IsTrue::True(50) => println!("True 50"),
    _ => println!("Not True"),
    IsTrue::False => println!("False"),
}
```

다시 [맨 처음의 `match` 예제](https://www.notion.so/Boom-Labs-Rust-1-6517b4b3ca354dc4bf3bef2bc3be7de0)로 돌아와서, 꼭 나머지 분기점을 다루지 않아도 된다면,

```rust
let my_value = IsTrue::True(100);

if let IsTrue::True(x) = my_value {
    println!("True {}", x);
}
```

위와 같이 `if let` 연산자를 이용할 수도 있습니다.

```rust
let my_value = IsTrue::True(50);

if let IsTrue::True(100) = my_value {
    println!("True");
} else if let IsTrue::True(x) = my_value {
    println!("True {}", x);
} else {
    println!("False");
}
```

`if let` 연산자는 `if` 문의 일종이기 때문에, 위와 같이 `else if` 및 `else`를 사용할 수도 있습니다.

Rust standard library에서 대표적으로 사용하는 Enum 타입으로는 `Option<T>`와 `Result<T, E>`가 있습니다. 
* 여기서 `T`나 `E`를 Generic type이라고 부르는데요, 이는 한 가지 데이터 타입이 아닌 여러 가지의 데이터 타입이 올 수 있는 경우에 처리하기 위한 타입입니다. 자세한 건 나중에 다시 다뤄보겠습니다.

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

[`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html)는 Rust에서 null의 상황을 표현할 때 사용됩니다. Rust는 언어 기본적으로 null type을 지원하지 않기 때문에 enum을 이용하여 처리합니다. 

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

[`Result<T, E>`](https://doc.rust-lang.org/std/result/enum.Result.html)는 Rust에서 에러 처리를 위해 사용되는 enum 타입입니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/items/enumerations.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “enums 1, 2, 3” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/enums/`

and you can verify your work by running `rustlings run enums1` command.
</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Match, if let](https://practice.rs/pattern-match/match-iflet.html)
</div></details>

### 1.6. Errors and Error handling

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

```rust
// git checkout 1.6.0
// rust-basic/src/main.rs
use std::error::Error;

fn process() -> Result<String, Box<dyn Error>> {
    Ok("Hello, world!".to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = process();

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }

    println!("{}", process()?);

    Ok(())
}
```

위의 절에서 말했듯이, Rust에서 에러를 처리하기 위해서는 `Result<T, E>` enum을 사용합니다.

여기서 `Result<T, E>` enum 내부의 에러와 관련된 Generic 타입을 보면, `Box<dyn Error>`라 써져있는 것을 확인할 수 있습니다. [`Box<T>`](https://rinthel.github.io/rust-lang-book-ko/ch15-01-box.html)는 struct 타입으로 힙에 있는 데이터를 가리키는 스마트 포인터입니다. 그 말은 `Box<>`로 감싸고 있는 타입은 항상 힙메모리에 저장되는 것이죠.

[`dyn`](https://doc.rust-lang.org/std/keyword.dyn.html) 키워드는 dynamically dispatched의 준말입니다. `dyn` 키워드는 Trait 타입을 수식하는 키워드인데요, 수식하는 Trait을 dynamically dispatched하도록 만듭니다. 

`Box` 및 `dyn`과 관련해서는 Part 2의 Polymorphism 세션에서 더 자세히 다루도록 하겠습니다.

여기서는 `Error` Trait을 소개하고자 이야기를 꺼냈습니다. Trait에 대한 자세한 내용은 Part 2에서 다루지만, 가볍게 소개하자면, 여러 타입들이 공통적으로 갖는 동작에 대하여 추상화하도록 해줍니다. Java의 interface나, c++의 abstract class와 유사한 기능을 합니다.

[`Error` trait](https://doc.rust-lang.org/nightly/core/error/trait.Error.html)은 에러를 핸들링하는 데에 필요한 공통 동작들을 추상화한 trait입니다.  
여기서는, `Result<T, E>` enum 타입 안의 `Err()` 타입에 씌워져있는 에러들이 `Error` trait으로 정의된다는 것이죠.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “errors 1, 2, 3, 4, 5, 6” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/error_handling/`

and you can verify your work by running `rustlings run errors1` command.
</blockquote></div></details>

### 1.7. Impl and methods

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

```rust
// git checkout 1.7.0
// rust-basic/src/main.rs
use std::error::Error;

struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    fn new_from_default() -> Employee {
        Employee {
            name: "default".to_string(),
            id: 100,
        }
    }

    fn new(name: String, id: u64) -> Employee {
        Employee { name, id }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();
    let employee2 = Employee::new("John".to_string(), 101);

    println!("{} {}", employee.name, employee.id);
    println!("{} {}", employee2.name, employee2.id);

    Ok(())
}
```

`impl` 키워드는 `Struct` 또는  `Trait` 타입의 공유 동작을 구현할 때 사용됩니다.  
위의 코드 예시에서는, `Employee` struct를 갖는 인스턴스들이 `new_from_default` 또는 `new` 메소드에 접근할 수 있도록 구현되어 있습니다.

각각의 공유 메소드는 인스턴스를 초기화시키는 데에 사용되는 메소드입니다.

이에 대해 더 자세한 예시는 아래 Reference 문서들을 확인해보시길 바랍니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/items/implementations.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “structs 3” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/structs/`

and you can verify your work by running `rustlings run structs3` command.
</blockquote></div></details> 

### 1.8. Ownership

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

Ownership, Borrowing, Lifetime은 Rust의 꽃과도 같은 개념입니다. Rust는 Gabage collector가 없지만, 이러한 개념을 통해 Rust 개발자가 메모리 누수에 깊이 고민하지 않더라도 메모리 안정성을 보장해줍니다. (적어도 컴파일이 되는 코드에 한해서)

이에 대해 더 자세한 예시는 아래 Reference에 링크걸어둔 Rust 교과서에 도표와 함께 잘 이해할 수 있도록 서술하고 있으니, 이 부분(1.8 절부터 1.10 절까지)은 꼭 교과서를 정독하길 바랍니다.

```rust
// rust-basic/src/main.rs
fn main() {
    let x = 10;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    let a = x;
    let b = s1;
}
```

위 코드를 실행시키면 에러가 발생합니다.

하지만 마지막 라인을 `// let b = s1;` 와 같이 주석처리를 해버리면, 정상적으로 동작함을 확인할 수 있습니다.

이 부분에 대해 의아하게 느껴질 수 있을텐데요, 한번 컴파일러가 뱉는 에러 코드를 확인해보겠습니다.

![Untitled 4](https://user-images.githubusercontent.com/96561121/201534113-9b6fc78e-df27-40bb-9e76-f802d7bae740.png)

보시면, 6번 라인에서 `s1`의 값이 `s2` 변수로 move되었는데, 9번 라인에서 이미 이동된 `s1`의 값을 사용하려고 했기 때문에 에러가 생겼다고 말하고 있습니다.

즉, `s1`이 갖고 있던 데이터 값에 대한 소유권이 `s2`에게로 이동(move)된 것입니다.  
포인터 개념에 익숙한 사람들의 경우엔 `s1`은 힙 메모리의 어딘가에 저장되어 있는 실제 문자열 값의 주소를 Pointing하고 있는데, 그 메모리 주소 값이 `s2`로 이동되었다고 이해할 수 있습니다.  
(이 부분은 Rust 교과서의 도표가 잘 설명하고 있기 때문에 꼭 교과서를 읽어주세요)

다만 `let a = x;` 처럼, `String` 타입이 아닌 `u64` 타입의 값의 경우엔 문제가 생기지 않았는데요.  
바로 이 부분에서 스택에 저장되는 데이터와 힙에 저장되는 데이터의 처리가 다름을 알 수 있습니다.

컴파일 타임에 할당해줘야 할 메모리의 크기가 정해지지 않는 데이터 타입들은 힙 메모리에 저장이 강제됩니다. `String` 타입이 그 예시인데, 런타임동안 구동되면서 동적으로 크기가 변하기 때문입니다.  
이러한 동적으로 변하는 타입의 경우, 실제 변수가 저장하고 있는 값은 힙 메모리 상에 존재하는 데이터의 메모리 주소입니다. 이 때 `let s2 = s1;`처럼, 다른 변수에 할당시키면, `s1`가 저장하고 있던 메모리 주소의 값이 `s2`으로 이동하는 것이죠.

`move`는 다른 언어에서의 포인터가 저장하고 있는 메모리 주소를 그대로 복사하는 `shallow copy` 와 유사한데요. 다만, 타 언어에서는 포인터의 주소를 복사한 후에 이전 포인터의 값도 그대로 접근이 가능한 반면, Rust는 이전 포인터는 무효화시키기 때문에 `move`라고 표현합니다.

`u64`와 같이 그 크기가 컴파일 타임에 결정되는 타입들은 스택 메모리에 저장됩니다. 이 스택 메모리에 저장되는 타입의 경우, (스택 메모리 특성 상) 데이터 값에 접근하는 것이 빠르기 때문에, 복사가 빠르게 이루어질 수 있습니다. 즉, `shallow copy`와 `deep copy`의 차이가 없습니다. 따라서 이러한 유형의 데이터 타입의 유형은 컴파일러가 `move`로 처리하지 않고, `copy`로 처리합니다.

힙 메모리에 저장되는 데이터도 다른 언어에서 처럼, 메모리에 저장되어 있는 데이터를 그대로 복사하여 새로운 메모리에 저장하는 `deep copy`를 수행할 수 있습니다.

```rust
// rust-basic/src/main.rs
fn main() {
    let x = 10;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();

    let a = x;
    let b = s1;
}
```

위 예시가 `.clone()`을 통해 `deep copy`를 하는 예시입니다.  
이 경우에는 `s1`이 저장하고 있는 메모리 주소가 `move`되지 않고, 메모리의 데이터를 그대로 복제한 후, `s2`에는 새로 복제한 메모리 데이터를 저장하기 때문에, `s1`값에 접근하더라도 컴파일 에러가 생기지 않습니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
</div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Ownership](https://practice.rs/ownership/ownership.html)
</div></details>

### 1.9. Borrowing and the borrow check

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

위 **1.8 절**의 예시에서, `.clone()`을 통해 `deep copy`를 하는 것 외에도, 값을 여러 변수에 넘겨주는 방법이 있습니다. 

```rust
// rust-basic/src/main.rs
fn main() {
    let x = 10;
    let y = x;

    let s1 = String::from("hello");
    let s2 = &s1;

    let a = x;
    let b = &s1;
    println!("{} {} {}", s1, s2, b);
}
```

이는 `s1`을 pointing하는 새로운 참조자를 만드는 것입니다.  
이 부분 역시 교과서에서 도표를 통해 잘 설명하고 있기 때문에, **[아래 Reference 문서](https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html)를** 정독 부탁드립니다.

참조자 및 가변 참조자 관련한 내용은 해당 문서에서 좋은 예제와 함께 잘 설명하고 있으니 생략하겠습니다.

```rust
use std::error::Error;

struct Employee {
    name: String,
    id: u64,
}

impl Employee {
    fn new_from_default() -> Employee {
        Employee {
            name: "default".to_string(),
            id: 100,
        }
    }

    fn new(name: String, id: u64) -> Employee {
        let employee = Employee { name, id };

        // println!("{}", name); // error[E0382]: borrow of moved value: `name`
        return employee;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

// &Employee is borrowed here
fn borrow_thing(employee: &Employee) {
    println!("borrowed: {}", employee.name);
} // borrowed &Employee is dropped here

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();
    let employee2 = Employee::new("John".to_string(), 101);

    borrow_thing(&employee);

    println!("{} {}", employee.name(), employee.id());
    println!("{} {}", employee2.name, employee2.id);

    Ok(())
}
```

잠시 우리가 만들던 예제 코드로 다시 돌아오겠습니다. `git checkout 1.9.0`을 입력하면 위와 같은 코드가 작성되어 있을 것입니다.

위 코드에서, `Employee` struct 안의 `name` 필드를 빌려와서 return하는 `name(&self)` 메소드와 유사한 기능의 `id(&self)` 메소드가 구현되어 있습니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “move_semantics 1, 2, 3, 4, 5, 6” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/move_semantics/`

and you can verify your work by running `rustlings run move_semantics1` command.
</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Reference and Borrowing](https://practice.rs/ownership/borrowing.html)
</div></details>

### 1.10. Lifetimes

<details>
<summary> Explanation </summary>
<div markdown="1"><blockquote>

위 코드의 `fn borrow_thing(employee: &Employee)` 함수를 고쳐보겠습니다. 위 코드는 Lifetime에 대한 내용이 생략되어 있는데, 생략된 내용을 상세히 풀자면 아래와 같습니다.

```rust
fn borrow_thing<'a>(employee: &'a Employee) -> &'a String {
    &employee.name
} 
```

Lifetime은 참조자가 유효한 scope를 표시해주는 지시자입니다.  
이에 대한 필요성을 알아보기 위해 아래와 같이 코드를 조금 변형해보겠습니다.

```rust
fn borrow_thing(one: &Employee, two: &Employee, boolean: bool) -> &String {
    if boolean {
        &one.name
    } else {
        &two.name
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();
    let employee2 = Employee::new("John".to_string(), 101);

    let name = borrow_thing(&employee, &employee2, true);

    println!("{} {} {}", name, employee.name(), employee.id());
    println!("{} {}", employee2.name, employee2.id);

    Ok(())
}
```

두 개의 `Employee` struct와 하나의 `bool` 을 받고, `bool` 의 값에 따라 인자로 받은 `Employee` struct 중에서 하나를 return하는 함수입니다.

![Untitled 5](https://user-images.githubusercontent.com/96561121/201534136-fec0e9ba-fe62-4fe9-ae4f-16830f6e95a9.png)

문제가 없어보이는 이 코드는 위와 같은 컴파일 에러를 표시합니다.

```rust
fn borrow_thing<'a, 'b>(one: &'a Employee, two: &'b Employee, boolean: bool) -> &'? String {
    if boolean {
        &one.name
    } else {
        &two.name
    }
}
```

이 코드 중 Lifetime이 생략된 부분을 보여주자면, 위와 같은 코드 형태가 될 것입니다. 문제는 여기서, 반환받는 타입의 참조자의 Lifetime은 bool값에 따라 `'a`가 될 수도, `'b`가 될 수도 있습니다.

Rust에서 각각의 참조자들이 어느 Scope에서 유효한지 명시하는 것은 댕글링 참조자를 피하기 위해 상당히 중요합니다. 

```rust
fn borrow_thing<'a>(one: &'a Employee, two: &'a Employee, boolean: bool) -> &'a String {
    if boolean {
        &one.name
    } else {
        &two.name
    }
}
```

컴파일러가 추천한 위 방식대로 코드를 짠 후, 돌리면 문제없이 코드가 실행됨을 알 수 있습니다.

여기서 참조자의 Lifetime을 `'a`로 통일하는 것은, `one`과 `two`, 그리고 return 값까지 모두 최소한 `'a`만큼의 Lifetime을 갖음을 보장한다는 계약입니다.

즉, `'a`는 `one`과 `two`의 Lifetime scope중 겹치는 영역이고, 적어도 return 타입은 이 영역에서는 유효하다는 의미입니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
</div></details>

### 1.11. Mutex and Arc

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html#smart-pointers)
- [std::sync::atomic - Rust](https://doc.rust-lang.org/std/sync/atomic/)
- [Arc in std::sync - Rust](https://doc.rust-lang.org/std/sync/struct.Arc.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote> 

Complete rustlings “arc 1” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/standard_library_types/`

and you can verify your work by running `rustlings run arc1` command.
</blockquote></div></details>
