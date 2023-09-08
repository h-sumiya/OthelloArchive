<script>
    export let lines = 2;
    export let select = null;
    export let check = null;
    export let mode = 1;
    let w = 100;

    $: len = w / lines;
    $: half = w / lines / 2;
    $: start = w / 2;
    $: path = "";
    $: {
        let d = "";
        for (let i = 0; i < lines; i++) {
            d += `M${len * i + half},100 L${start},0 `;
        }
        path = d;
    }
</script>

<div bind:clientWidth={w}>
    <svg viewBox="0 0 {w} 100">
        <path d={path} stroke="black" stroke-width="1" />
        {#if select !== null}
            <path
                d="M{len * select + half},100 L{start},0 "
                stroke={mode === 1 ? "red" : "blue"}
                stroke-width="3"
            />
        {/if}
    </svg>
    {#if check !== null}
        {#key check}
            <div class="line_box">
                <svg viewBox="0 0 {w} 100" class="line">
                    <path
                        d="M{start},0 L{len * check + half},100"
                        stroke="green"
                        stroke-width="3"
                    />
                </svg>
            </div>
        {/key}
    {/if}
</div>

<style>
    div {
        position: relative;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        z-index: -1000;
        height: 100px;
    }

    .line {
        position: absolute;
        top: 0;
        left: 0;
    }

    @keyframes line {
        0% {
            height: 0%;
        }
        100% {
            height: 100%;
        }
    }

    .line_box {
        position: absolute;
        overflow: hidden;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        z-index: -900;
        animation: line 0.5s linear;
    }
</style>
