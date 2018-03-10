http://www.nltk.org/book/ch01.html

## 1.1   Computing with Language: Texts and Words
```python3
from nltk.book import *

# concordance
# テキスト内の"monstrous"との一致を出力する
text1.concordance("monstrous")

# similar
# テキスト内の"monstrous"と同じ用法で使われている単語を出力する
text1.similar("monstrous")

# common_contexts
# 指定した単語が使われている文脈を出力する
text2.common_contexts(["monstrous", "very"])

# dispersion_plot
# 指定した単語がtext内にどのように出現するかをグラフにプロットする
text4.dispersion_plot(["citizens", "democracy", "freedom", "duties", "America"])

# generate
# テキスト内の単語でランダムな文章を生成する
# ※ NLTL3.0ではこのメソッドは使用できない（後に復活予定）

# len
# 単語数をカウント出来る
len(text3)

# set
# set（重複を許さないリスト）へ変換
# テキストのボキャブラリーを抽出できる

# 例：ボキャブラリーを照準で並べる
sorted(set(text3))

# 例：ボキャブラリー数をカウント
len(set(text3))

# すべての単語が平均何回出現するか
from __future__ import division
len(text3) / len(set(text3))

# aという単語がtextの単語の何%を占めるか
100 * text4.count('a') / len(text4)
```

## 1.3   Computing with Language: Simple Statistics
```
# frequency distribution

# FreqDist
# 頻度分布を簡単に行えるクラス
fdist1 = FreqDist(text1)

# 頻度の上位50
fdist1.most_common(50)
```

## 3.3   Collocations and Bigrams
```
# collocations
# テキスト内で一緒に使われている単語を抽出する
text4.collocations()
```