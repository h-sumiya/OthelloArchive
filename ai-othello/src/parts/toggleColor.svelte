<script>
    export let select = 1;
    export let disable = false;
</script>

<div class="wrap w_{select}">
    <div
        class="black"
        class:select={select == 1}
        on:click={() => (select = 1)}
        role="button"
        tabindex="0"
    >
        黒
    </div>
    <div
        class="white"
        class:select={select == 2}
        on:click={() => (select = 2)}
        role="button"
        tabindex="0"
    >
        白
    </div>
    {#if disable}
        <div class="cover" />
    {/if}
</div>

<style>
    .wrap {
        display: inline-block;
        border: solid black 1px;
        padding: 3px;
        border-radius: 9999px;
        transition: 0.3s;
        overflow: hidden;
        position: relative;
    }
    .w_1 {
        background-color: white;
    }
    .w_2 {
        background-color: black;
    }
    .black,
    .white {
        width: 100px;
        display: inline-block;
        text-align: center;
        border-radius: 9999px;
        transition: 0.3s;
        padding: 2px;
    }
    .black {
        color: white;
    }
    .black.select {
        background-color: black;
    }
    .white {
        color: black;
    }
    .white.select {
        background-color: white;
    }
</style>
