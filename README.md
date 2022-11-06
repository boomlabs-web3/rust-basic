# Part 2

## Theme: Object Oriented Programming features

### 2.0. Start of Part 2

```bash
git clone https://github.com/boomlabs-web3/rust-basic.git
cd rust-basic
git checkout 2.0.0
```

먼저 Part 2를 시작하기 전에, 이전 예제를 불러오겠습니다.
깃헙에 있는 rust-basic 예제 레포를 다운받지 않으신 분들은 위 커맨드를 통해 다운로드해주세요.

```rust
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

    // expanded or un-elided lifetime.
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let employee = Employee::new_from_default();

    println!("{} {}", employee.name(), employee.id());

    Ok(())
}
```

위는 Part 1의 개념들을 토대로 employee 들의 이름과 ID에 대한 struct를 구현한 기본적인 코드입니다.

`Employee` struct는 각각에 대응하는 데이터 값을 return하는 `name`과 `id`메소드를 implement하고 있습니다.

### 2.1. Traits

<details>
<summary> Explanation of Traits </summary>
<div markdown="1"> <blockquote>

“_트레잇은 다른 종류의 추상화를 사용할 수 있도록 해줍니다: 이는 타입들이 공통적으로 갖는 동작에 대하여 추상화하도록 해줍니다. 트레잇(trait) 이란 러스트 컴파일러에게 특정한 타입이 갖고 다른 타입들과 함께 공유할 수도 있는 기능에 대해 말해줍니다.”_

이번 시간에는 위 코드에 덧붙여서, `Product`라는 (상품들의 정보를 담는) 새로운 struct를 정의하고, 각 struct의 정보를 출력하는 공유 동작을 생각하고, 해당 공유 동작을 정의하는 `PrintInfo` 라는 trait을 정의하겠습니다.

```rust
struct Product {
    name: String,
    price: u64,
    quantity: u64,
    production_date: u64,
}

impl Product {
    fn new(name: String, price: u64, quantity: u64, production_date: u64) -> Product {
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
```

위는 새로 정의한 `Product` struct입니다.  
크게 다른 것은 없고, 상품에 대한 정보를 저장하고, `Product` struct에 대한 인스턴스를 initialize하는 `new` 메소드와,  
`production_date`를 보기 좋게 포매팅하는 `date` 메소드를 정의했습니다.

```rust
trait PrintInfo {
    fn print_info(&self);
}
```

다음으로, struct에 저장되어 있는 정보를 출력하는 공유 메소드를 구현해보겠습니다.

Rust의 trait은 Java의 interface와 유사합니다. 위와 같은 방식으로 공유 메소드인 `print_info` 를 정의할 수 있습니다.

```rust
impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(
            "employee's name: {}\nemployee's id: {}\n",
            self.name(),
            self.id()
        );
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

다음으로 `PrintInfo` trait에 정의되어 있는 `print_info` 공유 메소드를 각각의 struct에서 구현하는 부분입니다.

`impl {Trait} for {Struct}` 와 같은 문법을 사용하여 구현할 수 있습니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote>

Complete rustlings “traits 1, 2, 3, 4, 5” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/traits/`

and you can verify your work by running `rustlings run traits1` command.

</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Traits](https://practice.rs/generics-traits/traits.html)
</div></details>

### 2.2. Using Traits

<details>
<summary> Explanation of Using Traits </summary>
<div markdown="1"> <blockquote>

두 가지 방법으로 함수에서 Trait이 적용된 struct 타입의 객체를 받는 것을 알아보겠습니다.

이번에는 아래의 기능을 구현하는 `static_print` 함수를 정의해보겠습니다.

- `static_print` 함수는 `PrintInfo` trait이 implement된 struct의 인스턴스를 인자로 받겠습니다.
- `PrintInfo` trait 내의 `print_info` 메소드를 이용하여 인자로 받은 인스턴스의 정보를 출력하겠습니다.
- Struct type에 따라 다르게 정보를 출력하겠습니다.

```rust
// fn static_print<T: PrintInfo>(data: T)
// fn static_print<T>(data: T) where T: PrintInfo,
fn static_print(data: impl PrintInfo) {
    data.print_info();
}

fn dynamic_print(data: Box<dyn PrintInfo>) {
    // println!("Production date: {}", data.date());   // error[E0599]: no method named `date` found for struct `Box<(dyn PrintInfo + 'static)>` in the current scope
    data.print_info();
}

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);
    let product = Product::new("Apple".to_string(), 1, 100, 20220924);
    static_print(employee.clone());
    static_print(product.clone());
    dynamic_print(Box::new(employee));
    dynamic_print(Box::new(product));
}
```

**2.1. 절**의 코드에서 `static_print` 함수와 `dynamic_print` 함수를 추가했습니다. 이 함수의 인자를 보면, `PrintInfo` trait을 구현하고 있는 어떠한 struct type의 객체를 인자로 받겠다는 이야기입니다.

다만 그 문법에 조금 차이가 있는 것을 알 수 있습니다.

- `static_print` 함수는 정적 디스패치를 통해 구현되었습니다. 이는 어떤 메소드를 호출할 지, 컴파일 타임에 결정되어, 해당 결과가 컴파일된 실행 파일에 하드 코딩됩니다. 즉, `main` 함수 안의 3번째 라인에서는 `Employee` struct의 `print_info` 메소드가, 4번째 라인에서는 `Product` struct의 `print_info` 메소드가 불려질 것임이 실행파일에 하드 코딩됩니다.
- `dynamic_print` 함수는 동적 디스패치를 통해 구현되었습니다. 이는 `main` 함수 안의 5, 6번째 라인에서 어떤 메소드를 호출할 지에 대해, 컴파일 타임이 아닌 런타임에 결정됨을 의미합니다. 이에 대해서는 다음 절에서 더 자세히 다루겠습니다.

</blockquote></div></details>

### 2.3. Polymorphism in Rust

<details>
<summary> Explanation of Polymorphism in Rust </summary>
<div markdown="1"> <blockquote>

```rust
fn dynamic_print(data: Box<dyn PrintInfo>) {
    // println!("Production date: {}", data.date());   // error[E0599]: no method named `date` found for struct `Box<(dyn PrintInfo + 'static)>` in the current scope
    data.print_info();
}

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);
    let product = Product::new("Apple".to_string(), 1, 100, 20220924);

    dynamic_print(Box::new(employee));
    dynamic_print(Box::new(product));
}
```

`dynamic_print` 함수를 보면, `fn dynamic_print(data: Box<dyn PrintInfo>)` 와 같이 정의되어 있는데, 다시 한번 `Box` 와 `dyn` 키워드가 등장했습니다.

`Box<dyn T>` 문법은 Part 1에서 `Error` trait을 다룰 때 사용했었는데요. 이에 대해서는 아래 토글에서 자세히 다뤄보겠습니다.

<details>
<summary> Deep dive into static & dynamic dispatch of Rust </summary>
<div markdown="1">

Rust의 정적 디스패치는 높은 수준의 추상화가 컴파일 타임에 결정됩니다. 즉, Runtime-overhead가 없는 zero-cost abstractions을 제공합니다. 이는 Rust의 장점으로 여겨지는 특징 중 하나로, 이러한 것이 가능한 이유로는 Generic 타입이나 Trait을 사용하더라도, 컴파일러가 컴파일 과정에서 해당 target을 실행파일에 하드코딩합니다.

다만 Reference에 첨부한 미디엄 글에서의 예시처럼, 런타임에 trait이 구현되어 있는 특정 struct 타입을 밀어넣고, 그에 따른 function call이 달라지는 경우에는 정적 디스패치가 불가합니다. 이 경우엔 `dyn` 키워드를 사용하여, 성능을 조금 포기하면서도 동적 디스패치를 사용합니다.

_정적 디스패치 대비 동적 디스패치를 사용했을 때 성능 저하에 대한 벤치마크는 Reference의 미디엄 글을 확인하시길 바랍니다._

</div></details>

우리 예제로 돌아와서, `dynamic_print`함수는 `Employee` struct를 받는 지, `Product` struct를 받는 지에 따라 call하는 메소드가 달라집니다.

지금은 하드코딩하여 컴파일 타임에 넘겨주었지만, 런타임에서 위 값을 넘겨줄 수도 있습니다.

이를 위해 `dyn` 키워드를 사용하여 dynamic dispatch임을 알려줍니다. 또한 컴파일러가 메모리의 크기에 관해 불확정되었다는 불평을 피하기 위해, 우린 `Box<T>` struct에 감싸서 힙메모리에 넣어줍니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html)
- [Polymorphism in Rust](https://oswalt.dev/2021/06/polymorphism-in-rust/)
- [Polymorphism in Rust (KR)](https://modoocode.com/334)
- [Rust Dynamic Dispatching deep-dive](https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b)
</div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Trait Object](https://practice.rs/generics-traits/trait-object.html)
</div></details>

### 2.4. Crates and modules

<details>
<summary> Explanation of Crates and modules </summary>
<div markdown="1"> <blockquote>

마지막은 Rust 코드를 모듈화하는 방법에 대해 알아보겠습니다. `git checkout 2.4.0` 을 하시면 모듈화된 코드를 확인하실 수 있습니다.
이전까지는 main.rs 파일에 모든 코드를 저장했다면, struct와 trait별로 해당 코드를 쪼개어보겠습니다.

- main.rs

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

- printinfo.rs

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

- employee.rs

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

- product.rs

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

위는 모듈화한 코드인데요. 특별한 로직 추가 없이, main.rs에 모여있던 코드를 네 갈래로 쪼갠 것입니다. 코드를 보시면 `mod`, `pub`, `use` 키워드가 사용되었습니다.

- `mod`: 이 키워드는 새로운 모듈을 선언할 때에 사용됩니다. 본 예제처럼 다른 파일에 저장할 수도 있고, 아래와 같이 정의할 수도 있습니다.

  ```rust
  mod printinfo {
      pub trait PrintInfo {
          fn print_info(&self);
      }

      pub fn static_print(data: impl PrintInfo) {
          data.print_info();
      }

      pub fn dynamic_print(data: Box<dyn PrintInfo>) {
          data.print_info();
      }
  }
  ```

  또한 본 예제보다 더 깊은 계층구조를 갖는 경우, (모듈이 서브모듈을 갖는 경우)에는 mod.rs 에 모듈들 간의 연결고리를 몰아넣기도 합니다.
  이 부분은 이해하기에 어렵지 않으니, [Programming book](https://rinthel.github.io/rust-lang-book-ko/ch07-01-mod-and-the-filesystem.html)을 참고해주세요.

- `pub`: 키워드는 모듈 내의 구성요소를 외부에서 접근할 수 있도록 *가시적*으로 만듭니다. 기본적으로 함수, struct, trait 등의 구성요소들은 정의된 모듈 안에서만 접근할 수 있습니다. 따라서 모듈에 정의된 함수를 다른 모듈에서 호출하고자 하면 컴파일 에러가 생깁니다. 이를 `pub` 키워드를 앞에 붙이면 외부에서도 해당 구성요소에 접근할 수 있습니다.
- `use`: 외부 모듈의 구성요소를 참조할 때에, 해당 모듈의 path까지 입력해야 합니다. 예를 들면 `printinfo::static_print` 와 같이죠.
  다만 모듈의 path가 보다 복잡한 상황, 그리고 모듈 내의 구성요소가 여러번 호출될 때에는 코드가 지저분해지는 데, 그러한 상황에서 `use` 키워드를 사용하면 모듈이나 모듈 내의 정의들을 스코프 안으로 가져와서 이들을 더 쉽게 참조할 수 있도록 도와줍니다.  
  C++의 `using namespace` 와 유사합니다.

</blockquote></div></details>

<details>
<summary> Reference </summary>
<div markdown="1">

- [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/items/modules.html)
</div></details>

<details>
<summary> In-class Exercises (Rustlings) </summary>
<div markdown="1"> <blockquote>

Complete rustlings “modules 1, 2, 3” exercises.

Boilerplate codes are stored in path below,

`rustlings/exercises/modules/`

and you can verify your work by running `rustlings run modules1` command.

</blockquote></div></details>

<details>
<summary> In-class Exercises (Rust By Practice) </summary>
<div markdown="1">

- [Package and Crate](https://practice.rs/crate-module/crate.html)
</div></details>
