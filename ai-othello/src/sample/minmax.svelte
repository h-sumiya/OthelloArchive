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
    <div class="inner">
        {#key c}
            <MinmaxNode
                depth={2}
                start={true}
                on:value={cycle}
                n={Math.floor(Math.random() * 2) + 2}
            />
        {/key}
    </div>
</div>

<style>
    .wrap {
        height: 100%;
        margin: 0;
        position: relative;
        overflow-x: auto;
    }

    .inner {
        margin: 2.3rem 0;
        min-width: 700px;
    }
</style>
