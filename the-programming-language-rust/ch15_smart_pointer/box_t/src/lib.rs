// 15.1 Box<T>를 이용해 힙 메모리의 데이터 참조하기
// 박스(스마트 포인터)는 데이터를 스택이 아닌 힙 메모리에 저장한다.
// 스택에는 힙 데이터를 가르키는 포인터만 저장한다.

// 박스는 스택이 아닌 힙 메모리에 데이터를 저장한다는 것 외에는 성능 오버헤드가 없다.
// 박스를 사용하는 주요 예시
// - 컴파일 타임에 크기를 알 수 없는 타입을 정확한 크기가 필요한 상황에 사용하려고 할 때
// - 데이터의 크기가 커서 데이터를 복제하지 않고 소유권을 이전하고 싶을 때
// - 특정 타입이 아니라 특정 트레이트를 구현하는 타입의 값을 소유하고자 할 때

// 15.1.1 Box<T>를 이용해 힙 메모리에 데이터 저장하기
// Box<T>에 저장된 값을 다루는 방법과 문법
pub fn create_box() {
    let b = Box::new(5); // b의 데이터는 힙 메모리에 저장됨
    println!("b: {}", b);
}

// 15.1.2 박스를 이용해 재귀 타입 활용하기
// 러스트는 컴파일 타임에 어떤 타입이 얼마나 많은 메모리를 사용하는지 알아야 한다.
// 컴파일 타임에 크기를 알 수 없는 타입 중 하나는 재귀 타입(recursive type)이다.
// 재귀 타입이란, 같은 타입의 다른 값을 자신의 일부에 포함하는 값이다.

// ex) 함수형 프로그래밍 언어에서는 보편적인 데이터 타입인 '콘스 리스트(construct list)'
// (1) 리스트 생성자에 대한 더 자세한 정보
// 콘스 리스트는 리스프(Lisp) 프로그래밍 언어 및 파생 언어에 도입된 데이터 구조다.
// 'x를 y에 콘스'한다는 말은 원소 x를 y 리스트의 첫 부분에 추가해서 새로운 리스트를 생성한다는 의미이다.

// 콘스 리스트의 각 아이템은 현재 아이템의 값과 그 다음 아이템의 값 등 두 개의 원소로 구성된다.
// 리스트의 마지막 아이템은 다음 아이템이 없으므로 Nil이라는 값을 가진다.
// 이 리스트는 콘스 함수를 재귀적으로 호출해서 생성한다.

// 러스트는 컴파일 타임에 List에 어느 정도의 메모리를 할당해야할 지 알 수 없다.
// 그러나, 러스트는 Box<T>(즉, 스마트 포인터)가 어느 정도의 메모리를 차지하는지는 알 수 있다.
// 포인터의 크기는 포인터가 가르키는 값과 무관하게 변하지 않는다.
// Cons 열거값은 이제 i32 타입의 크기에 박스의 포인터 데이터를 저장할 공간만 있으면 된다!

// Box<T> 타입은 Deref 트레이트를 구현하고 있으므로, 스마트 포인터이며 참조와 같은 방법으로 사용할 수 있다.
// Drop 트레이트도 구현하고 있으므로, Box<T> 값이 범위를 벗어나면 박스가 가르키는 힙 메모리가 해제된다.
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

pub fn create_cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Nil)))));
}
