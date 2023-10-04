import init, { ai, version } from './lib/pkg';


let loaded = (async () => {
    await init();
    const result = version();
    console.log(`othello ai web edition(${result}) is successfully loaded.`);
    self.postMessage({ loaded: true })
})();

self.onmessage = async (e) => {
    await loaded;
    const { data, color } = e.data;
    const result = ai(data, color);
    self.postMessage(result);
};