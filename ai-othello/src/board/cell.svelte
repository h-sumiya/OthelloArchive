<script>
    import { createEventDispatcher } from "svelte";

    export let index;
    let dispatch = createEventDispatcher();
    function click() {
        dispatch("click", { index });
    }

    export let type = "null";
    export let latest = false;
    export let suggest = false;
    export let input = false;

    let before = null;
    $: if (type == "black") {
        before = "black";
    } else if (type == "white") {
        before = "white";
    } else {
        before = null;
    }

    $: _type = suggest
        ? type
        : type == "black" || type == "white"
        ? type
        : "null";
    $: click_able = (type == "legal_black" || type == "legal_white") && input;
</script>

{#if click_able}
    <div
        class="cell {_type} clickable"
        class:latest
        on:click={click}
        role="button"
        tabindex="0"
    />
{:else}
    <div class="cell {_type} " class:latest />
{/if}

<style>
    .cell {
        flex: 1;
        border: 1px solid black;
        background-color: green;
        position: relative;
    }

    .clickable {
        cursor: pointer;
    }

    @media (hover: hover) {
        .clickable:hover {
            background-color: rgb(27, 102, 27);
        }
    }
    @media (hover: none) {
        .clickable:active {
            background-color: rgb(27, 102, 27);
        }
    }

    .black::before,
    .white::before {
        position: absolute;
        content: "";
        display: block;
        width: 60%;
        height: 60%;
        margin: 20%;
        border-radius: 50%;
    }
    .black::before {
        background-color: black;
    }
    .white::before {
        background-color: white;
    }
    .legal_black::after,
    .legal_white::after {
        position: absolute;
        content: "";
        display: block;
        width: 20%;
        height: 20%;
        margin: 40%;
        border-radius: 50%;
    }
    .legal_black::after {
        background-color: black;
    }
    .legal_white::after {
        background-color: white;
    }
    .latest::after {
        position: absolute;
        content: "";
        display: block;
        width: 14%;
        height: 14%;
        margin: 43%;
        border-radius: 50%;
        background-color: red;
    }
</style>
