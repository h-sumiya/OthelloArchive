<script>
    import { main, initialize } from "./ai";
    import Spinner from "./parts/spinner.svelte";
    import Game from "./game.svelte";
    let init = false;
    (async () => {
        if (!init) {
            await main();
            init = true;
        }
    })();
</script>

{#await initialize}
    <Spinner text="AIを読み込んでいます." />
{:then}
    <Game />
{/await}
