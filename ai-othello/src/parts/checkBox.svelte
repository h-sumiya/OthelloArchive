<script>
    export let value = false;
    export let text;
</script>

<label class="label">
    <input type="checkbox" bind:checked={value} class="input" />
    <span class="checkmark" />
    <span class="span">{text}</span>
</label>

<style>
    .label {
        display: inline-block;
        position: relative;
        padding-left: 30px;
        margin-bottom: 12px;
        cursor: pointer;
        font-size: 14px;
        user-select: none;
        margin-right: 10px;
    }

    .input {
        display: none;
    }

    .checkmark {
        position: absolute;
        top: 0;
        left: 0;
        height: 20px;
        width: 20px;
        border: 2px solid #999999;
        box-sizing: border-box;
        border-radius: 4px;
    }

    .label .checkmark:after {
        content: "";
        position: absolute;
        display: none;
        left: 4px;
        top: 0px;
        width: 5px;
        height: 10px;
        border: 2px solid #fff;
        border-width: 0 2px 2px 0;
        transform: rotate(45deg);
    }

    .input:checked + .checkmark {
        background-color: #070707;
        border: 2px solid #070707;
    }

    .input:checked + .checkmark:after {
        display: block;
    }
</style>
