## Netflix Skip Intro

Windows UI Automation(UIA)로 넷플릭스 재생 중 나타나는 “Skip Intro(인트로 건너뛰기)” 버튼을 자동으로 찾아 클릭해 주는 작은 도구입니다.
브라우저(Edge/Chrome)에서 동작하도록 설계되었습니다.

⚠️ 개인용·연구용 데모입니다. 넷플릭스 UI/브라우저/OS 업데이트에 따라 동작이 바뀔 수 있습니다.
&nbsp;

### 특징

-   UIA를 이용해 버튼을 안전하게 클릭 (좌표 클릭이 아닌 접근성 트리 기반)
-   다국어 버튼 텍스트 매칭 지원하지 않음. 오직 한국어로 되어 있는 "넷플릭스", "오프닝 건너뛰기" 글자를 찾아서 동작함.
-   활성화 되어 있는 창의 화면을 주기적으로 스캔하여 버튼이 나타나면 즉시 동작

### 설치 & 실행

```bash
# 1) 저장소 받기
git clone https://github.com/JooJiyun/Netflix-skip-intro.git
cd Netflix-skip-intro

# 2) 빌드
cargo build --release

# 3) 실행
target/release/netflix_skip.exe
```

### 실행 파일

[release](https://github.com/JooJiyun/Netflix-skip-intro/releases)에서 최신 버전 exe파일 다운로드 후 실행

### 동작

해당 프로그램은 gui를 지원하지 않습니다.
실행하게 되면 시스템 트레이에 아이콘이 추가된 것을 확인할 수 있습니다.
해당 아이콘 우클릭으로 나오는 메뉴로 기능을 키거나 끌 수 있고 프로그램을 종료할 수 있습니다.
