<script>
    import MinmaxNode from "./minmaxNode.svelte";

    let c = 0;
    function cycle() {
        setTimeout(() => {
            c++;
        }, 5000);
    }
</script>

<div class="wrap">
    {#key c}
        <MinmaxNode
            depth={2}
            start={true}
            on:value={cycle}
            n={Math.floor(Math.random() * 2) + 2}
        />
    {/key}
</div>

<style>
    .wrap {
        height: 100%;
        margin: 2rem 20px; 
        position: relative;
    }
</style>
