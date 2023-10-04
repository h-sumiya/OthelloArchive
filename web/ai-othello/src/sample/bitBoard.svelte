<script>
    import BitIcon from "./bitIcon.svelte";
    import { onMount } from "svelte";

    class RandBoard {
        constructor() {
            this.black = 0n;
            this.white = 0n;
            for (let i = 0; i < 64; i++) {
                if (Math.random() < 1 / 3) {
                    this.black |= 1n << BigInt(i);
                } else if (Math.random() < 0.5) {
                    this.white |= 1n << BigInt(i);
                }
            }
        }
    }

    let data = new RandBoard();

    onMount(() => {
        let id = setInterval(() => {
            data.black = 0x7f7f7f7f7f7f7fn & (data.black >> 9n);
            if (data.black === 0n) {
                data = new RandBoard();
            }
        }, 1000);
        return () => clearInterval(id);
    });
</script>

<BitIcon n={data.black} color={1} />
