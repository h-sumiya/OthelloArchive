<script>
    import { onMount } from "svelte";
    let text = ".&nbsp;&nbsp;";
    onMount(() => {
        let i = 0;
        let id = setInterval(() => {
            i++;
            if (i > 3) {
                i = 0;
            }
            text = ".".repeat(i) + "&nbsp;".repeat(3 - i);
        }, 300);
        return () => {
            clearInterval(id);
        };
    });
</script>

{@html text}