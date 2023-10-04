<script>
    import { createEventDispatcher } from "svelte";
    import Bind from "./bind.svelte";
    import MinmaxNode from "./minmaxNode.svelte";
    import Value from "./value.svelte";

    export let n = Math.floor(Math.random() * 3) + 1;
    export let depth = 0;
    export let mode = 1;
    export let start = false;

    let dispatch = createEventDispatcher();

    let next = -1;
    let select = null;
    let check = null;
    let value = "";

    $: {
        if (start) {
            setTimeout(() => {
                next = 0;
                check = 0;
            }, 500);
        }
    }

    function batch(v, i) {
        if (value === "" || v * mode > value * mode) {
            select = i;
            value = v;
        }
        next++;
        if (next < n) {
            check = next;
        } else {
            check = null;
            dispatch("value", { value });
        }
    }
</script>

<div class="wrap" class:view={depth === 0 && next >= 0 && next !== n}>
    <div class="value"><span>{value}</span></div>
    <Bind lines={n} {check} {select} {mode} />
    <div class="wraps">
        {#if depth !== 0}
            {#each { length: n } as _, i}
                <MinmaxNode
                    depth={depth - 1}
                    mode={mode * -1}
                    on:value={(e) => batch(e.detail.value, i)}
                    start={i <= next}
                />
            {/each}
        {:else}
            {#each { length: n } as _, i}
                <Value
                    on:value={(e) => batch(e.detail.value, i)}
                    start={i <= next}
                />
            {/each}
        {/if}
    </div>
</div>

<style>
    .wrap {
        width: 100%;
        overflow: visible;
        margin: none;
        position: relative;
    }

    .view {
        flex-grow: 0;
    }

    .wraps {
        display: flex;
        justify-content: space-around;
        margin: 1em 0;
        margin: 0;
        overflow: visible;
    }

    .value {
        position: absolute;
        height: 2em;
        width: 2em;
        top: 0;
        left: 50%;
        border: solid 1px;
        background-color: white;
        text-align: center;
        border-radius: 50%;
        transform: translateY(-50%) translateX(-50%);
    }

    .value > span {
        line-height: 1.8em;
    }
</style>
