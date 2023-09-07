<script>
    import { createEventDispatcher } from "svelte";
    let dispatch = createEventDispatcher();
    export let start = false;

    let value = "";
    let done = false;
    $: {
        if (start && !done) {
            done = true;
            setTimeout(() => {
                value = value = Math.floor(Math.random() * 100 - 50);
                dispatch("value", { value });
            }, 500);
        }
    }
</script>

<div class="box">
    <div class="value">
        <span>{value}</span>
    </div>
</div>

<style>
    .box {
        width: 100%;
        overflow: visible;
        margin: none;
        position: relative;
    }
    .value {
        position: absolute;
        height: 2em;
        width: 2em;
        bottom: 0;
        left: 50%;
        border: solid 1px;
        background-color: white;
        text-align: center;
        border-radius: 50%;
        transform: translateY(50%) translateX(-50%);
    }
    span {
        line-height: 1.8em;
    }
</style>
