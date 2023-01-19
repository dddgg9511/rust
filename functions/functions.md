# functions
``` rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```
- 함수 선언은 `fn`으로 시작하며 함수 이름 뒤에 괄호의 형식으로 되어 있다.
- 함수의 이름과 괄호 형식을 기입하는 것을 통해 선언했던 함수를 호출할 수 있다.

## parameters

``` rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```
- 함수는 매개변수를 갖는 형식으로 선언될 수 있다.
- 함수가 매개변수를 취할 때 상수를 전달인자로 제공할 수 있다.
- 
--- 

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

여러개의 매개변수를 사용하고 싶으면 `,`와 함께 구분해서 사용할 수 있다.

## Statements and Expressions

함수 본문은 필요에 따라 표현식으로 종결되는 구문의 나열로 구성된다.

- `statements` : 명령들의 나열로 값을 반환하지 않는 동작을 수행한다.
- `Expressions` : 결과 값을 산출해낸다.

```rust
fn main() {
    let y = 6;
}

```
함수 정의 또한 `Statements`이다.

```rust
fn main() {
    let x = (let y = 6);
}
```
`y = 6` 은 값을 반환하지 않기 때문에 `x`에 bind 시킬 것이 없다.

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

Expression block
```rust
{
    let x = 3;
    x + 1
};
```
위 `block`은 `4`를 반환한다. 
> `Expressions`는 종결을 나타내는 `:`을 사용하지 않는다. `;`를 추가하면 `Statements`로 변경되어 값으로 평가받지 않는다.


## Functions with Return Values

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

- 함수는 호출한 코드에 값을 반환할 수 있다.
- 반환 값의 타입을 -> 뒤에 선언해야 한다.
- rust 에서 반환 값은 함수 본문의 마지막 `statements`의 값과 동일하다.
- `return` 키워드와 값을 써서 바로 반환할 수 있지만, 대부분의 함수들은 암묵적으로 마지막 `statements`를 반환한다.