<script>
    export let text = "Hello World!";
    export let value = "Hello World!";
</script>

<div class="info">
    <input type="text" bind:value placeholder="input u64 number or hex" />
    <div>:</div>
    <div>{text}</div>
</div>

<style>
    .info {
        margin-top: 20px;
        text-align: center;
    }
    .info > input,
    .info > div {
        display: inline-block;
        line-height: 1em;
    }
</style>
