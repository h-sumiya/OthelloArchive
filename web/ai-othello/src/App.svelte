<script>
    import { initialize } from "./ai";
    import Spinner from "./parts/spinner.svelte";
    import Game from "./game.svelte";
    import Body from "./body.svelte";
    import Footer from "./footer.svelte";
    import Header from "./header.svelte";
    import Support from "./support.svelte";
</script>

<Header />
<Support />
{#await initialize}
    <Spinner text="AIを読み込んでいます" />
{:then}
    <Game />
{/await}

<Body />
<Footer />
