<script>
    const ua = window.navigator.userAgent.toLowerCase();
    const inChrome = ua.indexOf("chrome") > -1;
    const inFirefox = ua.indexOf("firefox") > -1;
    const isSafari = ua.indexOf("safari") > -1 && !inChrome && !inFirefox;
    const isIE =
        ua.indexOf("msie") > -1 ||
        (ua.indexOf("trident") > -1 && !inChrome && !inFirefox);
    const isNotSupported = isSafari || isIE;

    let name = "";
    if (isSafari) {
        name = "Safari";
    } else if (isIE) {
        name = "Internet Explorer";
    }
</script>

{#if isNotSupported}
    <div class="warp">
        <div>
            ブラウザに
            <div class="name">{name}</div>
            をお使いの場合、一部の機能が正しく動作しない可能性があります。
        </div>
        <div>
            画面が正しく表示されない場合はChrome, Firefox,
            Edgeなどの最新のブラウザをご利用ください。
        </div>
    </div>
{/if}

<style>
    .warp {
        margin-top: 10px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .warp > div {
        border-bottom: solid red 1px;
    }

    .name {
        font-weight: bold;
        display: inline;
    }
</style>
