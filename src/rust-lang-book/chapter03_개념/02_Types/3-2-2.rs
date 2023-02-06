/*
스칼라 타입들

스칼라는 하나의 값으로 표현되는 타입입니다.
정수형, 부동소수점 숫자, boolean, 문자 4가지를 가지고 있습니다.

## 정수형

정수형은 소수점이 없는 숫자입니다.

|Length|Signed|Unsigned|
|-|-|-|
|8 bit|i8(-,+)|u8(only +)|
|16 bit|i16|u16|
|32 bit|i32|u32|
|64 bit|i64|u64|
|arch|isize|usize|

#### 정수형 리터럴

const x = 1;

x는 상수이고 1이 리터럴이다.

|Number literals|Example|
|-|-|
|Decimal|98_222|
|Hex|0xff|
|Octal|0o77|
|Binary|0b1111_0000|
|Byte (u8 only)|b'A'|

## 부동 소수점 타입

Rust에서는 소수점을 갖는 숫자인 부동소수점 숫자를 위한 두가지 기본타입이 있다.

`f32`와 `f64`가 있고 기본 타입은 `f64`입니다.

CPU상에서는 f64가 f32와 대략 비슷한 속도를 내면서 더 정밀한 표현이 가능하기 때문이다.

*/

fn main(){
    let x = 2.0; //f64
    let y :f32 = 3.0; //f32

    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

}