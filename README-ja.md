## 遊び方
基本的にはリバーシのルールと同じです。
黒が先手で置きたいコマを選択し、置き場所を選んで(置くことができるマスは黄色で表示されます)クリックすることでコマを置くことができます。

通常のリバーシとは以下の点が異なります。
- コマには1~10の数が振られています。相手のコマを挟んだとき、自分の両端のコマの合計よりも小さいコマだけをひっくり返せます。
ただし、相手のコマを挟めるときは、相手のコマが合計よりも大きいものしかない場合でも、コマを置くことができます。
- 勝敗は各色のコマの数ではなく、各コマの数字の合計で決まります。
- 置く場所がない場合はパスとなり、相手の手番に自動で移ります。そのとき、ペナルティとして相手の使ったコマの中で最も大きい数字のコマの数が1つ増えます。


- ゲーム開始
- Boardの生成
- 初期配置を生成
- 終了判定
    - PassStateが(True, True)

- 手番
    - 初期値は黒(Black)
    - 設置可能場所の抽出(設置可能なマスをPuttableにする) fn(turn, Board) -> Option(Puttable)
        - 全てのマスについて反転可能なコマが存在するか検索する fn(Position, Board) -> Option(Candidate)
        - 反転可能なコマが存在すればPuttableに格納する fn(Option(Candidate)) -> Option(Puttable)

        - 設置可能な場合(Puttableが存在する)
            - 石の選択
            - 設置場所の選択
            - 石を置く
                - 石の数が0ではない fn(piece) -> Option(ExistPiece)
                - 設置可能な場合(選択したマスがPuttableである) fn(position, Puttable) -> Option(PuttablePosition)
                    - Boardの更新
                        - 設置場所をputに変える fn(piece, position) -> Board
                        - 石を反転させる fn(board, piece, candidate) -> Board
                            - 両端の合計を求める fn(piece, candidate) -> usize
                            - 合計より値が小さい相手コマを抽出する fn(sum, candidate) -> Option(target)
                            - targetがある場合
                                - 抽出したコマの色を自分の色に変更する fn(board, target) -> board
                            - targetがない場合
                                - 何もしない
                    - 選択した石を１減らす fn(piece, ExistPiece) -> piece
                    - 相手の手番に変更(Black -> White / White -> Black) fn(turn) -> turn
                    - PassStateをFalseにする fn(passstate) -> passstate
                - 設置不可能な場合
                    - 何もしない
        - 設置不可能な場合(Puttableが存在しない)
            - パス
                - 相手側の使用済み石の中から最大値を検索 fn(Used) -> Piece
                - 持ち石を１増やす fn(Piece) -> Piece
                - 使用済み石からその石を削除 fn(Used) -> Used
                - 相手の手番に変更(Black -> White / White -> Black)
                - PassStateをTrueにする fn(passstate) -> passstate