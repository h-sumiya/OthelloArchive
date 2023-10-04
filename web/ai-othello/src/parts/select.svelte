<script>
    export let select = null;
    export let values = [];

    function click(index) {
        if (index == select) {
            select = null;
        } else {
            select = index;
        }
    }
</script>

<div class="wrap">
    {#each values as value, i}
        <div
            class="item"
            on:click={() => {
                click(i);
            }}
            role="button"
            tabindex="0"
            class:select={i == select}
        >
            {value}
        </div>
    {/each}
</div>

<style>
    .wrap {
        text-align: center;
        margin-bottom: 10px;
    }
    .item {
        display: inline-block;
        margin: 5px 10px;
        padding: 5px 15px;
        border: 1px solid #424242;
        border-radius: 10px;
        cursor: pointer;
    }

    @media (hover: hover) {
        .item:hover {
            background-color: #424242;
            color: white;
        }
    }
    @media (hover: none) {
        .item:focus {
            background-color: #424242;
            color: white;
        }
    }
    .select {
        background-color: #5f5f5f;
        color: white;
    }
</style>
