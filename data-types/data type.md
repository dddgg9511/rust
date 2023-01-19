# 데이터 타입들

## Scalar type
- 하나의 값으로 표현되는 타입

### Integer Types
- 소수점이 없는 숫자

#### rust 에서의 정수 타입

| Length | Signed | Unsigned |
|:-------|:-------|:---------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |


### Floating-Point Types
- f32 : 1배수의 정밀도인 부동소수점
- f64 : 2배수의 정밀도인 부동소수점

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### Boolean Type

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

- 다른 언어들과 마찬가지로 true, false 두 개의 값을 가질 수 있다.

### Character Type

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

- rust 에서 `char` 타입은 `Unicode Scalar`를 표현하는 값

## Compound Types
- 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있다

### Tuple Type
- 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입으로 만드는 일반적인 방법

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
`tuple`은 `single compound element`로 고려되었기 때문에 `tuple` 전체가 하나의 변수에 `bind` 된다.  

---

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
개별 값을 `tuple`의 밖으로 빼내오기 위해서는 패턴 매칭을 사용하여 `tuple`의 값을 구조해체 시키면 된다.

---
```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

`.` 을 사용하여 접근하길 원하는 값의 `index`fmf 넣는 것을 통해 `tuple`의 요소에 직접적으로 접근할 수 있다.

### Array Type
- `tuple`과는 다르게 `array`의 모든 요소는 모두 같은 타입이어야 한다.
- rust 에서는 `array`가 고정된 길이를 갖는다. 한번 선언되면 커지거나 작아지지 않는다.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
대 괄호 안에 값들을 `,`로 구분하여 나열해서 배열을 만든다.

---
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
배열은 `stack`에 single `chunk` 로 할당되고 `index`를 사용하여 배열의 요소에 접근할 수 있다.

---
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```
위와 같이 배열의 크기를 넘어선 `index`에 접근하려고 하면 `Runtime Error`가 발생한다.