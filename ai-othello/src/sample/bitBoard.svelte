<script>
    import BoardIcon from "./boardIcon.svelte";

    class RandBoard {
        constructor() {
            this.me = 0n;
            this.op = 0n;
            for (let i = 0; i < 64; i++) {
                if (Math.random() < 0.5) {
                    this.me |= 1n << BigInt(i);
                } else {
                    this.op |= 1n << BigInt(i);
                }
            }
        }
    }
    let rand = new RandBoard();
    let mask = 0x7E7E7E7E7E7E7E7En & rand.me;
</script>





