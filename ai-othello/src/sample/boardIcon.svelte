<script>
    export let data;
    export let w = 8;
    export let h = 8;

    $: ws = w / 8;
    $: hs = h / 8;
</script>

<div class="icon" style:--ws={ws} style:--hs={hs}>
    {#each { length: h } as _, y}
        <div class="row">
            {#each { length: w } as _, x}
                <div class="cell cell_{data[y * w + x]}" />
            {/each}
        </div>
    {/each}
</div>

<style>
    .icon {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        width: calc(min(200px, 40vw) * var(--ws));
        height: calc(min(200px, 40vw) * var(--hs));
    }

    .row {
        display: flex;
        width: 100%;
    }

    .cell {
        flex-grow: 1;
        border: 1px solid black;
        background-color: green;
        position: relative;
    }

    .cell_1::after,
    .cell_2::after {
        content: "";
        position: absolute;
        margin: 20%;
        width: 60%;
        height: 60%;
        border-radius: 50%;
        display: block;
    }

    .cell_1::after {
        background-color: black;
    }

    .cell_2::after {
        background-color: white;
    }

    .cell_3 {
        background-color: transparent;
        border: none;
    }
</style>
