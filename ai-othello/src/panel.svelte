<script>
    import { createEventDispatcher } from "svelte";
    import CheckBox from "./parts/checkBox.svelte";
    import ToggleColor from "./parts/toggleColor.svelte";
    let dispatch = createEventDispatcher();
    function reset(p) {
        dispatch("reset", { p });
    }
    export let display_ai_legal = true;
    export let display_player_legal = true;
    export let player = 1;
    export let free = true;
    $: reset(player);
</script>

<div class="panel">
    <div class="r_wrap">
        <div
            class="reset"
            role="button"
            tabindex="0"
            on:click={() => {
                if (free) {
                    reset(player);
                }
            }}
        >
            <span>はじめから</span>
            {#if !free}
                <div class="cover" />
            {/if}
        </div>
    </div>
    <div class="title">候補を表示</div>
    <div class="inner">
        <CheckBox text="プレイヤー" bind:value={display_player_legal} />
        <CheckBox text="AI" bind:value={display_ai_legal} />
    </div>
    <div class="title">色を選択する(ゲームがリセットされます。)</div>
    <div class="inner">
        <ToggleColor bind:select={player} disable={!free} />
    </div>
</div>

<style>
    .panel {
        width: 100%;
    }

    @media (width >= 700px) {
        .panel {
            max-width: 500px;
            min-width: 500px;
        }
    }

    .title {
        margin-top: 20px;
        padding-left: 10px;
        border-bottom: solid 1px gray;
    }

    .inner {
        margin-left: 20px;
        margin-top: 10px;
    }

    .r_wrap {
        text-align: center;
        margin: 20px 0 10px 0;
    }

    .reset {
        display: inline-block;
        background-color: rgb(10, 10, 10);
        color: rgb(243, 243, 243);
        padding: 3px;
        border-radius: 9999px;
        transition: 0.2s;
        overflow: hidden;
        cursor: pointer;
        padding: 3px 20px;
        position: relative;
    }

    @media (hover: hover) {
        .reset:hover:not(:disabled) {
            background-color: rgb(63, 63, 63);
        }
    }
    @media (hover: none) {
        .reset:active:not(:disabled) {
            background-color: rgb(63, 63, 63);
        }
    }

    .cover {
        position: absolute;
        width: 100%;
        height: 100%;
        left: 0;
        top: 0;
        background-color: rgba(105, 105, 105, 0.5);
    }
</style>
