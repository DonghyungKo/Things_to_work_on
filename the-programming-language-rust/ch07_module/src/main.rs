// 사용자 정의 모듈에서 함수를 불러온 후 실행
use ::restaurant::eat_at_restaurantcar_imported;
fn main() {
    eat_at_restaurantcar_imported();
}

// 핵심 개념
// - 패키지
// - 크레이트
// - 라이브러리
// - 작업공간(workspace)
// - 범위(Scope): 중첩된 context, 특정 범위 안에 작성된 코드는 해당 범위 안에서만 유효하다.

// 모듈 시스템의 구성gi
// - 패키지: 크레이트를 빌드, 테스트, 공유할 수 있는 카고의 기능
// - 크레이트: 라이브러리나 실행 파일을 생성하는 모듈의 트리(tree)
// - 모듈과 use: 코드의 구조와 범위, 그리고 경로의 접근성을 제어하는 기능
// - 경로(path): 구조체, 함수, 혹은 모듈 등의 이름을 결정하는 방식

//
// 7.1 패키지와 크레이트

// 크레이트
//  - 하나의 바이너리 혹은 라이브러리
//  - 크레이트 루트(root)는 러스트 컴파일러가 컴파일을 시작해서 크레이트의 루트 모듈을 만들어 내는 소스 파일이다.

// 패키지
// 일련의 기능을 제공함
// 하나 이상의 바이너리 크레이트 혹은 라이브러리 크레이트를 포함한다.

// cargo new 명령어를 실행하면 Cargo.toml 파일을 생성해 패키지를 만들어 낸다.
// 그러나, Cargo.toml 파일을 보면 src/main.rs 파일에 대한 언급이 없음..?
// 그 이유는 src/main.rs 파일은 패키지와 같은 이름을 갖는 바이너리 크레이트의 크레이트 루트라는 규칙을 알고 있기 때문
// 마찬가지로 카고는 패키지 디렉토리에 src/lib.rs 파일이 있으면 이 패키지는 패키지와 같은 이름의
// 라이브러리 크레이트를 포함한다고 판단하며, src/lib.rs 파일을 크레이트 루트로 인식한다.
// 카고는 라이브러리나 바이너리를 빌드할 때 rustc 컴파일러에게 크레이트 루트 파일을 전달한다.

// 만약 패키지 내에 src/main.rs 파일과 src/lib.rs 파일을 모두 가진다면 이는 라이브러리와 바이너리 크레이트를 모두 가진다는 의미
// 두 크레이트 이름은 모두 패키지 이름과 같다.
// 패키지의 src/bin 디렉토리에 파일을 분산해서 여러 개의 바이너리 크레이트를 추가할 수도 있다.
// 이떄 각 디렉토리의 파일들은 별개의 바이너리 크레이트가 된다.

// 크레이트는 관련된 기능들을 하나의 범위로 그룹화하므로, 해당 기능을 여러 프로젝트에서 공유하기가 수월해진다.
// 예를 들어, rand 크레이트는 난수를 생성하는 기능을 제공한다.

// 크레이트는 각각 고유한 namespace를 가지므로, 이름 충돌을 해결할 수 있다.

//
// 7.2 모듈을 이용한 범위와 접근성 제어
// - use, pub, as, glob

// 모듈(module)은 크레이트의 코드를 그룹화해서 가독성과 재사용성을 향상하는 방법이다.
// 또한, 아이템의 공개/비공개 여부를 결정한다.

// 레스토랑 기능을 제공하는 간단한 바이너리 크레이트를 예제
