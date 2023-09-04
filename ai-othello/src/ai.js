import wasm from "../../webai/Cargo.toml";

let app = null;
export async function init() {
    const exports = await wasm({
        serverPath: "/build/assets/",
    });
    let varsion = exports.varsion();
    console.log("ai version:", varsion);
    app = exports
}

export function ai(data, color) {
    return app.ai(data, color);
}

export function moves(data, color) {
    return app.legal_moves(data, color);
}

export function flip(data, loc, me, opp) {
    return app.put(data, loc, me, opp);
}