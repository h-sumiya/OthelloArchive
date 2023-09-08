<script>
    import BoardIcon from "./boardIcon.svelte";

    export let data = 0n;
    export let color = 1;

    function to_list(data) {
        let list = [];
        for (let i = 0; i < 64; i++) {
            if (data & (1n << BigInt(i))) {
                list.push(i);
            }
        }
        return list;
    }
</script>

