<script>
    import Loading from "./loading.svelte";
    export let text;
</script>

<div class="loading">
    <div class="spinner" />
    <div>{text}<Loading /></div>
</div>

<style>
    .loading {
        position: relative;
        text-align: center;
        padding: 20px 0;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .spinner {
        width: 100px;
        height: 100px;
        border-radius: 50%;
        border: 10px solid #ccc;
        border-top-color: #333;
        animation: spin 1s ease infinite;
        display: inline-block;
    }
</style>
