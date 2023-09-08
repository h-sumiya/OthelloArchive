/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function version(): string;
/**
* @param {Uint32Array} data
* @param {number} ai
* @returns {number}
*/
export function ai(data: Uint32Array, ai: number): number;
/**
* @param {Uint32Array} data
* @param {number} me
* @returns {Uint32Array}
*/
export function legal_moves(data: Uint32Array, me: number): Uint32Array;
/**
* @param {Uint32Array} data
* @param {number} pos
* @param {number} me
* @param {number} opp
* @returns {Uint32Array}
*/
export function put(data: Uint32Array, pos: number, me: number, opp: number): Uint32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly version: (a: number) => void;
  readonly ai: (a: number, b: number, c: number) => number;
  readonly legal_moves: (a: number, b: number, c: number, d: number) => void;
  readonly put: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
