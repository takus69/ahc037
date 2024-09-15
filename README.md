# AHC037

## 考察

- (0, 0)から順に全種類作ってもコストは同じ。それがコストが最小
- 一気に(a, b)を作るのと、(a, 0)->(a, b)作るのでコストは一緒
- マンハッタン距離が小さい方から処理していくのが良いか？
- 最適なコストは一番近くて、(0, 0)側にいる炭酸飲料から作成する。2点間の内側で距離が近いコストを合計する
- 理想的なコストでスコア計算してみたけど、順位表を見るとそれよりもいい！？ => なぜ？問題文をもう一度読む！
  - 途中経過の炭酸飲料が最終結果の炭酸飲料より近くにある場合は、それから作成した方が効率が良くなる
- 二段階でなく、途中で作成していくとより効率が良くなるはず。ただしやりすぎると5Nに収まらない

## 履歴

- v1: (0, 0)から(a, 0)を作成。その次(a, b)を作成(スコア: 299,710,044, 予測: 300,153,234)
- v2: すでに作成した炭酸飲料のうち一番近いものから作成する。作成時は2段階で作成する(スコア: 3,835,181,323, 予測: 3,813,228,750)

## 実行コマンド

- 1テストケース実行

```
cargo build
cat .\in\0000.txt | .\target\debug\ahc037.exe > .\out\0000.txt
.\tools\vis.exe .\in\0000.txt .\out\0000.txt
```

- 一括実行

```
cargo build
python .\simulator.py
```
