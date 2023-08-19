<script>
    import Cell from "./cell.svelte";

    export let data;

    let width;
    let height;
    $: size = Math.min(width, height) / 8;
</script>

<div
    class="board"
    style:--size="{size}px"
    bind:clientHeight={height}
    bind:clientWidth={width}
>
    {#each data as row, y}
        <div class="row">
            {#each row as cell, x}
                <Cell bind:select={data[y][x]} />
            {/each}
        </div>
    {/each}
</div>

<style>
    .board {
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        flex-direction: column;
    }

    .row {
        display: flex;
        justify-content: center;
        flex-direction: row;
    }
</style>
