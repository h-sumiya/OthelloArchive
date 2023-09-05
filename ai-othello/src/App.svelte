<script>
    import { main } from "./ai";
    import Board from "./board/board.svelte";
    let init = false;
    (async () => {
        if (!init) {
            await main();
            init = true;
        }
    })();
</script>

{#if !init}
    <div class="loading">
        <div class="spinner" />
    </div>
{:else}
    <Board />
{/if}
