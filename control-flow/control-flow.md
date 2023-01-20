# control-flow

## `if` Expressions
`if` 표현식은 코드가 조건에 따라 분기할 수 있게 한다.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```
- `else` 키워드는 `if` 식의 조건이 `false`인 경우 실행될 코드 블록을 제공한다.
- `if`삭에 제공되는 조건은 반드시 `boolean`타입이어야 한다.

--- 

### Handling Multiple Conditions with else if

```rust
fn main() {
let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

- `else if` 키워드는 `else` 키워드에 조건을 추가하여 사용할 수 있다.
- `if` 식을 차례대로 검사하고 첫 번째 참인 조건만 실행한다.

### Using if in a let Statement
`if`가 표현식이기 때문에 `let`구문의 우측에 사용할 수 있다.

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

변수 `number`에는 `if`식에서 산출된 값이 바운딩된다.

---
```rust
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```
`if`와 `else`분기의 값 타입이 호환지 않는 다면 `Runtime Error`가 발생한다.


## Repetition with Loops

### Repeating Code with `loop`

`loop` 키워드는 명시적으로 중단 점을 선언하기 전까지 코드 블록을 반복 수행한다.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

#### Returning Values from `Loops`
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

- `break` 키워드를 사용하여 `loop`반복을 멈출 수 있다.
- `break` 키워드 뒤에 값을 사용하여 반환한다.

#### `Loop Labels` to Disambiguate Between Multiple Loops
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

- 선택적으로 `lable`을 지정하여 해당 코드 블록의 `loop`대신 `lable`이 지정된 루프에 적용할수 있다.

### Conditional Loops with `while`
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```
- `while`은 조건이 `true`인 동안 코드가 실행되고 그렇지 않으면 루프에서 벗어난다.

### Looping Through a Collection with `for`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

`for` 반복문을 사용하여 `collection`의 요소들을 순회할 수 있다.