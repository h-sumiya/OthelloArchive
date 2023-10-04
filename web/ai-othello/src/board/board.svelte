<script>
    import { legal_moves } from "../logic";
    import Cell from "./cell.svelte";

    export let data;
    export let player = 1;
    export let latest = null;
    export let input = false;
    export let suggest = true;
    export let size = 500;

    let cell_data;
    const dcell = { type: "null", latest: false, suggest_type: "null" };
    $: {
        let result = [];
        let legal = legal_moves(data, player);
        for (let i = 0; i < data.length; i++) {
            if (data[i] == 0) {
                if (legal.includes(i)) {
                    let suggest_type = player == 1 ? "black" : "white";
                    result.push({
                        ...dcell,
                        type: "null",
                        latest: false,
                        suggest_type,
                    });
                } else {
                    result.push({ ...dcell, type: "null" });
                }
            } else if (data[i] == 1) {
                result.push({ ...dcell, type: "black" });
            } else if (data[i] == 2) {
                result.push({ ...dcell, type: "white" });
            }
        }
        if (latest != null) {
            result[latest].latest = true;
        }
        cell_data = result;
    }
</script>

<div class="board" style:width="{size}px" style:height="{size}px">
    {#each { length: 8 } as _, y}
        <div class="row">
            {#each { length: 8 } as _, x}
                <Cell
                    {...cell_data[y * 8 + x]}
                    {suggest}
                    {input}
                    index={y * 8 + x}
                    on:click
                />
            {/each}
        </div>
    {/each}
</div>

<style>
    .board {
        display: flex;
        flex-direction: column;
    }

    .row {
        display: flex;
        flex-direction: row;
        flex: 1;
    }
</style>
