<script>
    import Loading from "./parts/loading.svelte";
    export let counts = [2, 2];
    export let player = 1;
    export let next = 1;
    export let finish = false;
    export let size = 500;

    $: label =
        player == 1 ? ["黒(あなた)", "白(AI)"] : ["黒(AI)", "白(あなた)"];
    $: msg = finish
        ? counts[player - 1] > counts[2 - player]
            ? "あなたの勝ち"
            : counts[player - 1] < counts[2 - player]
            ? "AIの勝ち"
            : "引き分け"
        : next == player
        ? "あなたの番です。"
        : "AIが考えています";

    $: load = next !== player && !finish;
</script>

<div class="info" style:width="{size}px">
    {label[0]}: {counts[0]} | {label[1]}: {counts[1]} | {msg}
    {#if load}
        <Loading />
    {/if}
</div>

<style>
    .info {
        text-align: center;
        display: inline-block;
    }
</style>
