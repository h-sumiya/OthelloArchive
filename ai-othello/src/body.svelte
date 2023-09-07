<script>
    import Title from "./parts/title.svelte";
    import Select from "./parts/select.svelte";
    import Minmax from "./sample/minmax.svelte";

    import Book from "./sample/book.svelte";
    import Score from "./sample/score.svelte";
    let select = null;
</script>

<div class="body">
    <Title>このサイトについて</Title>
    <div class="inner">
        <p>
            このサイトは上智大学のプログラミングサークル(UnbiasedProgramming)内のイベントProjectSoma2023で作成したオセロAIをブラウザ上で動かせるように移植したものです。
        </p>
        <p>
            サイトのソースコードは<a
                href="https://github.com/h-sumiya/OthelloAI-WEB">こちら</a
            >にあります。
        </p>
        <p>
            移植前のオセロAIは<a
                href="https://www.codingame.com/multiplayer/bot-programming/othello-1"
                >オセロAIの大会</a
            >に提出し最高で18位の成績を残しています。
        </p>
    </div>
    <Title>サイトの仕組み</Title>
    <div class="inner">
        <p>
            オセロAIのコードはRustを使って書かれており、WebAssemblyに変換して静的なページとして公開しています。
        </p>
        <p>UI部分はSvelteを使用し作成しました。</p>
        <Select values={["Rust", "WebAssembly", "Svelte"]} bind:select />
        {#if select === 0}
            <p>
                今回のプロジェクトでは主にRustというプログラミング言語を使い開発しました。
            </p>
            <p>
                Rustは数あるプログラミング言語の中でも最も高いパフォーマンスを発揮するこのできる言語の一つであり、今回のようにブラウザ上で実行することができるなど、ほぼすべてのプラットフォームに向けてコンパイルすることもできます。
            </p>
            <p>
                更にRustでは独自の仕組みによりプログラミング中意識せずとも安全とパフォーマンスが保証されます。
            </p>
            <p>
                入門に最適なチュートリアルはこちら<a
                    href="https://doc.rust-jp.rs/book-ja/">こちら</a
                >
            </p>
        {:else if select === 1}
            <p>
                WebAssemblyとは従来ブラウザ上ではJavaScriptのみ実行できたのに対し、Rust、C、GOなどの言語を用いて書かれたコードを
                コンパイルし生成されたバイナリをブラウザ上で実行できるようにする技術です。
            </p>
            <p>
                JavaScriptと比較して高速に動作し、今回のように大量の盤面を探索する際には非常に有効です。
            </p>
        {:else if select === 2}
            <p>
                Svelteとは独自の記法で書いたSvelteファイルをCSS、JavaScriptにコンパイルするツールです。
            </p>
            <p>
                WebフレームワークとしてはReact、Vue、Angularなどが有名ですが、Svelteはこれらのツールと比較して少ないコード量、小さなコンパイルサイズ、高速な動作が特徴です。
            </p>
            <p>
                今回のプロジェクトでは短い期間での開発に大きく貢献してくれました。
            </p>
            <p>
                入門に最適なチュートリアルはこちら<a
                    href="https://svelte.jp/tutorial/basics">こちら</a
                >
            </p>
        {/if}
    </div>
    <Title>AIの仕組み</Title>
    <div class="inner">
        <p>
            このオセロAIは主に4つの要素、定跡データ、αβ法、評価関数、Bitboardによって構成されています。
        </p>
        <p>
            AIははじめに定跡データの確認を行い登録されている手が見つかった場合に登録されていた手を打ちます。
        </p>
        <div>
            <Book />
            <div class="info">
                図のように盤面の状態をハッシュテーブルのキーとしてその後打つべき手を値に格納している
            </div>
        </div>
        <p>
            AIは後述するαβ法によって最善手を探索しますが、8手ほど先までのみの探索となるので序盤においては終盤を見据えた手を打つことができません。
        </p>
        <p>
            そこで序盤において取ることのできるパターンが少ないことを利用して事前に打つ手を決めて置くと言うことです。
        </p>
        <p>
            登録されている定跡が見つからなかった場合にAIはαβ法による探索を開始します。αβ法とはより簡単なアルゴリズムであるミニマックス法と同じ結果を高速に出すことができるアルゴリズムです。
        </p>
        <p>
            ミニマックス法とは自分は常に最善手を打ち、相手は常に自分にとって最悪の手を打つという仮定のもとに探索を行い、その中で最も良い結果となる手を探索するアルゴリズムです。
        </p>
        <Minmax />
        <div class="info">
            赤線(自分のターン)は最大値、青線(相手のターン)は最小値を探している。
        </div>
        <p>
            では評価関数とは何を行うものでしょうか?
            前述のミニマックス法やαβ法では最悪の手、最善の手をそれぞれ打つと説明しましたが何を持って最善の手、最悪の手とするのでしょうか?
        </p>
        <p>
            この問題の答えが評価関数です。評価関数ではある盤面の状態に対して点数をつけます。その点数を評価値と呼び探索においてはこの点数をもとに最善の手を決定します。
        </p>
        <p>
            しかしオセロの盤面に決まった点数、良し悪しはありません。そこでこの評価関数をどのように作成するかが様々なオセロAIの差となります。
        </p>
        <p>
            今回作成したAIでは1ターン目から60ターン目の最終点差をPyTorchを用いて学習させたニューラルネットワークによって評価値を算出しています。
        </p>
        <Score />
        <div class="info">
            実際には10個のマスなら3^10通りのパターンしかないため盤面を細かく分割して学習させ、それぞれのパターンの点数を事前に計算しておくことで高速化を図っていいる。
        </div>
        <p>
            最後にBitboardについて軽く説明します。Bitboardとは盤面の状態を2つの64bitの整数で表現したものです。
        </p>
        <p>
            コンピューターでは数字を01000111...のように0と1のみを使って管理していてbitとは一つの0か1のことを指します。
        </p>
        <p>
            64bitの整数は64個のbitを持っているので64マスの盤面を表現することができ、黒のデータ、白のデータの2つを用意することで盤面の状態を表現することができます。
        </p>
        <p>
            このような方法を用いる理由は、詳しい説明は省略しますが一度にすべての着手可能なマスを求めたり、着手後の盤面を求める処理を非常に高速に行うことが可能になるためです。
            更に2つの数字で1ターン分のデータを表すことができるためメモリの使用量も少なく済みます。
        </p>
        <p>
            以上が簡易的なこのオセロAIの仕組みになります。詳しい説明はソースコードを参照してください。
        </p>
    </div>
</div>

<style>
    .body {
        width: 100%;
        max-width: 1200px;
        margin: 100px auto 200px auto;
    }

    p {
        margin: 0 0 10px 0;
    }

    .inner {
        padding: 5px 20px 10px 20px;
    }
    .info {
        text-align: center;
        font-size: small;
        margin-bottom: 20px;
        padding: 0 20px;
    }

    @media (width < 1180px) {
        .body {
            padding: 0 20px;
        }
    }
</style>
