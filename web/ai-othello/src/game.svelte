<script>
    import Panel from "./panel.svelte";
    import Board from "./board/board.svelte";
    import Info from "./info.svelte";
    import { default_data } from "./store";
    import { flip, have_legal_moves } from "./logic";
    import { ai } from "./ai";

    let size = 500;

    let player = 1;
    let latest = null;
    let finish = false;

    let data = default_data;

    function reset() {
        data = default_data;
        latest = null;
        finish = false;
        next = 1;
    }

    let cpu_setting = {
        suggest: true,
        input: false,
        player: 3 - player,
    };
    let player_setting = {
        suggest: true,
        input: true,
        player: player,
    };
    $: player_setting.player = player;
    $: cpu_setting.player = 3 - player;

    let next = player;
    $: setting = next == player ? player_setting : cpu_setting;

    async function ai_turn() {
        let index = await ai(data, next);
        latest = index;
        data = flip(data, index, next);
    }

    function legal() {
        return have_legal_moves(data, next);
    }
    function opp_legal() {
        return have_legal_moves(data, 3 - next);
    }

    let useing = false;
    async function select(e) {
        if (useing) {
            return;
        }
        useing = true;
        let index = e.detail.index;
        latest = index;
        data = flip(data, index, next);
        next = 3 - next;
        while (legal()) {
            await ai_turn();
            if (opp_legal()) {
                break;
            }
        }
        next = 3 - next;
        if (!legal() && !opp_legal()) {
            finish = true;
        }
        useing = false;
    }

    $: counts = [
        data.filter((x) => x == 1).length,
        data.filter((x) => x == 2).length,
    ];

    async function new_game() {
        reset();
        if (player == 2) {
            await ai_turn();
            next = 3 - next;
        }
    }
</script>

<div class="game">
    <div class="main" bind:clientWidth={size}>
        <Board {...setting} {data} {latest} on:click={select} {size} />
        <Info {next} {counts} {size} {finish} {player} />
    </div>
    <Panel
        bind:display_ai_legal={cpu_setting.suggest}
        bind:display_player_legal={player_setting.suggest}
        bind:player
        on:reset={new_game}
        free={player == next}
    />
</div>

<style>
    .game {
        width: 100%;
        position: relative;
        margin-top: 20px;
    }
    .main {
        width: 100%;
        min-width: 300px;
    }

    @media (width >= 700px) {
        .game {
            display: flex;
            justify-content: center;
            align-items: center;
            flex-wrap: wrap;
        }

        .main {
            max-width: 500px;
            margin: 0 20px 0 20px;
        }
    }
</style>
