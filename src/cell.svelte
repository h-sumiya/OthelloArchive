<script>
    export let select = false;
    $: className = select ? "cell select" : "cell empty";
</script>

<div class={className} on:click={() => (select = !select)} />

<style>
    .cell {
        width: var(--size);
        height: var(--size);
        border: solid 1px black;
        position: relative;
        background-color: green;
        transition: 0.2s;
    }

    .cell::after {
        content: "";
        display: block;
        position: absolute;
        top: 20%;
        left: 20%;
        width: 60%;
        height: 60%;
        border-radius: 50%;
        transition: 0.2s;
    }

    .empty:hover::after {
        background-color: rgb(116, 116, 116);
    }

    .select::after {
        background-color: rgb(12, 12, 12);
    }

    .select:hover::after {
        background-color: rgb(46, 46, 46);
    }
</style>
