<script>
    import { createEventDispatcher } from "svelte";

    export let index;
    let dispatch = createEventDispatcher();
    function click() {
        if (click_able) {
            dispatch("click", { index });
        }
    }

    export let type = "null";
    export let latest = false;
    export let suggest = false;
    export let input = false;
    export let suggest_type = "null";

    let animation = "";
    let before = null;
    $: if (type == "black") {
        if (before == "white") {
            animation = "white_to_black";
        } else {
            animation = "";
        }
        before = "black";
    } else if (type == "white") {
        if (before == "black") {
            animation = "black_to_white";
        } else {
            animation = "";
        }
        before = "white";
    } else {
        animation = "";
        before = null;
    }

    $: click_able = suggest_type != "null" && input;
    $: s_type = suggest ? `s_${suggest_type}` : "s_null";
</script>

{#if click_able}
    <div
        class="cell {type} clickable suggest {s_type}"
        class:latest
        on:click={click}
        role="button"
        tabindex="0"
    />
{:else}
    <div class="cell {type} {animation} suggest {s_type}" class:latest />
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
    .suggest::after {
        position: absolute;
        content: "";
        display: block;
        width: 20%;
        height: 20%;
        margin: 40%;
        border-radius: 50%;
        transition: 0.5s;
    }
    .s_null::after {
        background-color: transparent;
    }
    .s_black::after {
        background-color: black;
    }
    .s_white::after {
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
    @keyframes black_to_white {
        0% {
            background-color: black;
            transform: rotateY(0deg);
        }
        50% {
            background-color: black;
            transform: rotateY(90deg);
        }
        50.1% {
            background-color: white;
            transform: rotateY(90deg);
        }
        100% {
            background-color: white;
            transform: rotateY(180deg);
        }
    }
    @keyframes white_to_black {
        0% {
            background-color: white;
            transform: rotateY(0deg);
        }
        50% {
            background-color: white;
            transform: rotateY(90deg);
        }
        50.1% {
            background-color: black;
            transform: rotateY(90deg);
        }
        100% {
            background-color: black;
            transform: rotateY(180deg);
        }
    }
    .black_to_white::before {
        animation: black_to_white 0.5s;
    }
    .white_to_black::before {
        animation: white_to_black 0.5s;
    }
</style>


