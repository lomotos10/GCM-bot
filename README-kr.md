# GCM-bot: 게키츄마이 채보 정보 전달 디스코드 봇 (온게키 WIP)

봇 초대 링크: https://discord.com/api/oauth2/authorize?client_id=986651489529397279&permissions=2147502080&scope=applications.commands%20bot

## 사용 방법

방법 1. 슬래시 명령어 (추천 방법)

방법 2. @GCM-bot `명령어-이름` `명령어-변수`

**노래 제목으로는 한글 제목 및 영어 별명들이 지원됩니다. 이것저것 시도해 보세요!**

**사용 예시:**

/mai-info 브브브

@GCM-bot mai-info 새벽까지 앞으로 3초

/mai-info bbb

/mai-info gc

봇을 서버에서 사용하는 것 이외에도, 봇에게 DM을 보내는 것으로 동일한 응답을 받을 수 있습니다.

## 제목 건의 / 개발자 문의

요청하시는 기능이 있거나, 추가하고 싶은 노래 제목 별명이 있다면 다음 방법 중 하나로 개발자에게 연락하실 수 있습니다:

1. 디스코드 @Lomo#2363 나 수정사항 제보용 채널 https://discord.gg/8tVDqfZzAN 로 질문/요청사항 보내기
2. 이 레포에 이슈 올리기
3. 이 레포에 pull request 요청

노래 별명은 `data/aliases/{locale}/{game}.tsv` 폴더에 위치해 있습니다. tsv 파일의 각 줄은 tab-separated lines로 구성되어 있고, 한 라인은 노래 하나에 해당합니다.

각 라인의 첫 번째 항목은 원제목이고, 그 뒤로는 노래 별명들이 쭉 나열됩니다. 별명을 추가하고 싶으면, 탭으로 띄어쓰기를 하면서 라인 뒤에 계속 추가하시면 됩니다.

`data/aliases/kr/maimai.tsv` 예시 라인:
```
渦状銀河のシンフォニエッタ	나선은하	와상은하	신포니에타	심포니에타	나선은하의 신포니에타
```
위에서 말한 대로, 첫 번째 항목이 원제목이고, 그 뒤로 탭으로 띄어쓰기를 한 별명들이 나열됩니다. 이 예시에서는 별명이 5개가 있는 것을 확인할 수 있습니다.

## TODO

- [ ] Remove hard panics and unwraps
- [ ] Add Chuni
- [ ] Add Ongeki
