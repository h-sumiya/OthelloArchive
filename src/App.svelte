<script>
	import Board from "./board.svelte";
	import Info from "./info.svelte";
	import { dataToBit, textToData } from "./lib/api";
	let data = [...Array(8)].map(() => [...Array(8)].map(() => false));

	let text = "0x00";
	const set_text = async (data) => {
		let d = await dataToBit(data);
		text = `${d.text}`;
	};

	let value = "";
	const set_data = async (value) => {
		data = await textToData(value);
	};

	$: set_data(value);
	$: set_text(data);
</script>

<div class="board">
	<Board bind:data />
</div>
<Info {text} bind:value/>

<style>
	.board {
		margin-top: 10px;
		width: 100%;
		height: calc(100vh - 6em);
	}
</style>
