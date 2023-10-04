<script>
    import { onMount } from "svelte";
    import BoardIcon from "./boardIcon.svelte";
    import Icons from "../parts/icons.svelte";
    class Book {
        constructor(me, opp, pos) {
            this.data = [];
            for (let i = 0; i < 64; i++) {
                let mask = 1n << BigInt(i);
                if (me & mask) {
                    this.data.push(1);
                } else if (opp & mask) {
                    this.data.push(2);
                } else {
                    this.data.push(0);
                }
            }
            this.pos = pos;
        }
    }
    const Books = [
        new Book(0x00081820000000n, 0x0010001e180000n, "f6"),
        new Book(0x10302020200400n, 0x04081c1c181800n, "d1"),
        new Book(0x00381200000000n, 0x00040c1c000000n, "b6"),
        new Book(0x00221400000800n, 0x0808083c3c3000n, "c2"),
        new Book(0x0000002c0c0800n, 0x00083e10000000n, "b6"),
        new Book(0x00003008080000n, 0x003c0c14240000n, "e3"),
        new Book(0x00003800000000n, 0x0000001e100000n, "d3"),
        new Book(0x0028000c102000n, 0x00043c10000000n, "f4"),
        new Book(0x000018180c0000n, 0x00382024000000n, "c5"),
        new Book(0x003c7c38002000n, 0x00010246fe1800n, "e1"),
        new Book(0x0000060e0c2c20n, 0x143c3830301010n, "d1"),
        new Book(0x00000018180000n, 0x00203c24040000n, "d6"),
        new Book(0x00000810100000n, 0x00203028000000n, "d3"),
        new Book(0x00102074000400n, 0x000e1c08380800n, "b5"),
        new Book(0x000018181c0000n, 0x00040406000000n, "b3"),
        new Book(0x18002010000000n, 0x20f85808000000n, "f8"),
        new Book(0x04083c04040000n, 0x00000038380000n, "g3"),
        new Book(0x0008043a080000n, 0x00003804060000n, "e6"),
        new Book(0x081828000a0000n, 0x0004143e302000n, "e2"),
        new Book(0x0000682e0a0000n, 0x003c1450341000n, "b6"),
        new Book(0x002c0e0c040000n, 0x10103030182800n, "g4"),
        new Book(0x00080008000000n, 0x00101c10000000n, "f4"),
        new Book(0x0000181c040000n, 0x00382020080000n, "e3"),
        new Book(0x00040000040010n, 0x00383838180804n, "f3"),
        new Book(0x0000040c040000n, 0x00181810000000n, "f6"),
        new Book(0x00080020080000n, 0x0010381c040000n, "b3"),
        new Book(0x00201038102000n, 0x000e0c060f0000n, "a4"),
        new Book(0x00181818080000n, 0x20202400100000n, "g6"),
        new Book(0x101c1000700000n, 0x20002e7c080000n, "f6"),
        new Book(0x1010181c181000n, 0x00002020200000n, "g4"),
        new Book(0x00081c001c0000n, 0x0004003e000000n, "b5"),
        new Book(0x000010001c0000n, 0x0000081c200000n, "d6"),
    ];
    let index = 0;
    onMount(() => {
        let id = setInterval(() => {
            let temp = Math.floor(Math.random() * Books.length);
            while (temp == index) {
                temp = Math.floor(Math.random() * Books.length);
            }
            index = temp;
        }, 600);
        return () => clearInterval(id);
    });
</script>

<div class="book">
    <BoardIcon data={Books[index].data} />
    <div class="then"><Icons name="then" /></div>
    <div class="num">{Books[index].pos}</div>
</div>

<style>
    .book {
        margin: 10px;
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
    }
    .num {
        min-width: 3rem;
        font-size: large;
    }
    .book > div {
        margin: 5px;
    }
    .then {
        width: 2rem;
        height: 2rem;
    }
</style>
