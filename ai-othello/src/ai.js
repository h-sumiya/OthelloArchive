import ai_worker from './ai.worker?worker';


const worker = new ai_worker();
export let initialize = new Promise((resolve, reject) => {
    worker.onmessage = (e) => {
        resolve()
    }
});


const WAIT = 1000;
export async function ai(data, color) {
    const start = Date.now()
    worker.postMessage({ data, color })
    const result = await new Promise((resolve, reject) => {
        worker.onmessage = (e) => {
            resolve(e.data)
        }
    })
    const elapsed = Date.now() - start;
    console.log(`ai elapsed: ${elapsed}ms`);
    if (elapsed < WAIT) {
        await new Promise((resolve, reject) => {
            setTimeout(() => {
                resolve()
            }, WAIT - elapsed)
        })
    }
    return result
}