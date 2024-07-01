# Moelyrics

一个简易的Moegirl歌词爬取器 ***请勿滥用***

目前仅测试了部分歌词 (不保证站上的所有歌词都遵循一定格式)：
- [以恋结缘](https://mzh.moegirl.org.cn/%E4%BB%A5%E6%81%8B%E7%BB%93%E7%BC%98)
- [DESIRE](https://mzh.moegirl.org.cn/DESIRE)
- [吉他与孤独与蓝色星球](https://mzh.moegirl.org.cn/%E5%90%89%E4%BB%96%E4%B8%8E%E5%AD%A4%E7%8B%AC%E4%B8%8E%E8%93%9D%E8%89%B2%E6%98%9F%E7%90%83)

## 提示

```shell
Usage: moelyrics.exe [OPTIONS] --url <URL> --output <FILE PATH> --hiragana <MODE>

Options:
  -u, --url <URL>
  -o, --output <FILE PATH>  Output .html file path
      --romaji              Display Romaji below lyric lines
      --translation         Display Chinese Translation below lyric lines
      --hiragana <MODE>     Display Hiragana above lyric lines with specific mode [possible values: tips, pure, hidden]
  -h, --help                Print help
  -V, --version             Print version
```

## 示例

```shell
PS C:\CodeSpace\moelyrics\target\debug> .\moelyrics.exe -u https://mzh.moegirl.org.cn/DESIRE -o a.html --romaji --hiragana tips --translation
```

Output:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Moelyrics</title>
</head>
<body>
    <div><ruby>夜空<rt>よぞら</rt></ruby><ruby>舞<rt>ま</rt></ruby>う<ruby>粉<rt>こな</rt></ruby><ruby>雪<rt>ゆき</rt></ruby>のような<br/>yo zo ra ma u ko na yu ki no yo u na<br/>粉雪夜空中起舞</div>
    <br/>
    <div><ruby>降<rt>ふ</rt></ruby>りしきる<ruby>桜<rt>さくら</rt></ruby><ruby>吹雪<rt>ふぶき</rt></ruby>でも<br/>hu ri si ki ru sa ku ra hu bu ki de mo<br/>樱吹雪簌簌飘落</div>
    <br/>
    <div><ruby>言<rt>い</rt></ruby>いたげな お<ruby>空<rt>そら</rt></ruby>の<ruby>月<rt>つき</rt></ruby>でも<br/>i i ta ge na  o so ra no tu ki de mo<br/>欲言又止的明月啊</div>
    <br/>
    ...
</body>
</html>
```