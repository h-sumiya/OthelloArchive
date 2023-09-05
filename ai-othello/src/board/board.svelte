<script>
    import { legal_moves } from "../lib/pkg/webai";
    import Cell from "./cell.svelte";

    export let data = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    export let player = 1;
    export let latest = null;
    export let input = false;
    export let suggest = true;

    let cell_data;
    $: {
        let result = [];
        let legal = legal_moves(data, player);
        for (let i = 0; i < data.length; i++) {
            if (data[i] == 0) {
                if (legal.includes(i)) {
                    let type = player == 1 ? "legal_black" : "legal_white";
                    result.push({ type, latest: false });
                } else {
                    result.push({ type: "null", latest: false });
                }
            } else if (data[i] == 1) {
                result.push({ type: "black", latest: false });
            } else if (data[i] == 2) {
                result.push({ type: "white", latest: false });
            }
        }
        if (latest != null) {
            result[latest].latest = true;
        }
        cell_data = result;
    }
</script>

<div class="board">
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
        width: 500px;
        height: 500px;
        flex-direction: column;
    }

    .row {
        display: flex;
        flex-direction: row;
        flex: 1;
    }
</style>
