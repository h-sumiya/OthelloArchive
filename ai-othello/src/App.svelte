<script>
    import { main, initialize } from "./ai";
    import Spinner from "./parts/spinner.svelte";
    import Game from "./game.svelte";
    import Body from "./body.svelte";
    import Footer from "./footer.svelte";
    import Header from "./header.svelte";

    //temp
    let init = false;
    (async () => {
        if (!init) {
            await main();
            init = true;
        }
    })();
    //temp
</script>

<Header />

{#if init}
    {#await initialize}
        <Spinner text="AIを読み込んでいます" />
    {:then}
        <Game />
    {/await}
{/if}

<Body />
<Footer />
