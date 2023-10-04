<script>
    import Icons from "../parts/icons.svelte";
    import BoardIcon from "./boardIcon.svelte";
    import { onMount } from "svelte";

    function rand_data() {
        let data = [];
        for (let i = 0; i < 64; i++) {
            data.push(Math.floor(Math.random() * 3));
        }
        return data;
    }

    const corner = [0, 1, 2, 3, 8, 9, 10, 16, 17, 24];
    const corner_index = [0, 1, 2, 3, 4, 5, 6, 8, 9, 12];
    const cross = [0, 9, 18, 27, 36, 45, 54, 63];
    class Board {
        constructor(data) {
            this.data = data;
            this.rands = [];
            for (let i = 0; i < 3; i++) {
                this.rands.push(Math.floor(Math.random() * 64 - 32));
            }
        }

        edge() {
            return this.data.slice(0, 8);
        }

        corner() {
            let data = Array(16).fill(3);
            for (let i = 0; i < corner.length; i++) {
                data[corner_index[i]] = this.data[corner[i]];
            }
            return data;
        }

        cross() {
            let data = Array(64).fill(3);
            for (let i = 0; i < cross.length; i++) {
                data[cross[i]] = this.data[cross[i]];
            }
            return data;
        }
    }

    let data = new Board(rand_data());
    onMount(() => {
        let id = setInterval(() => {
            data = new Board(rand_data());
        }, 1000);
        return () => clearInterval(id);
    });
</script>

<div class="src"><BoardIcon data={data.data} /></div>
<div class="flow flow_m">
    <Icons name="then" />
</div>
<div class="parts">
    <div>
        <BoardIcon data={data.cross()} />
        <div class="flow">
            <Icons name="then" />
        </div>
        <div class="rand">A:{data.rands[0]}</div>
    </div>
    <div>
        <BoardIcon data={data.corner()} w={4} h={4} />
        <div class="flow">
            <Icons name="then" />
        </div>
        <div class="rand">B:{data.rands[1]}</div>
    </div>
    <div>
        <BoardIcon data={data.edge()} w={1} />
        <div class="flow">
            <Icons name="then" />
        </div>
        <div class="rand">C:{data.rands[2]}</div>
    </div>
</div>
<div class="res">
    A({data.rands[0]}) + B({data.rands[1]}) + C({data.rands[2]}) = SCORE:{data
        .rands[0] +
        data.rands[1] +
        data.rands[2]}
</div>

<style>
    .res {
        text-align: center;
        margin-bottom: 10px;
    }
    .src {
        margin-top: 10px;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    }

    .flow_m {
        margin: 0 auto;
    }

    .parts {
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
    }

    .parts > div {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .flow {
        width: 2rem;
        height: 2rem;
        transform: rotate(90deg);
    }

    .rand {
        display: inline-block;
        margin: 0 auto;
    }
</style>
