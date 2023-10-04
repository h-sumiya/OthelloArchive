<script>
    import BoardIcon from "./boardIcon.svelte";

    export let n = 0n;
    export let color = 1;

    function to_arr(n, color) {
        let result = [];
        for (let i = 0; i < 64; i++) {
            if (n & (1n << BigInt(i))) {
                result.push(color);
            } else {
                result.push(0);
            }
        }
        return result;
    }

    $: arr = to_arr(n, color);
</script>

<div class="icon">
    <BoardIcon data={arr} />
    <div>10進数表記{n}</div>
    <div>16進数表記:{n.toString(16)}</div>
</div>

<style>
    .icon {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .icon > div {
        display: inline-block;
        font-size: small;
    }
</style>
