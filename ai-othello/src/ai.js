import init, { version } from './lib/pkg'

let loaded = false;
export async function main() {
    if (loaded) return;
    await init()
    const result = version()
    console.log(`othello ai web edition(${result}) is successfully loaded.`)
    loaded = true;
}