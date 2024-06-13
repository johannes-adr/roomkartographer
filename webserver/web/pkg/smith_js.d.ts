/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_wasm(): void;
/**
*/
export class SmithJS {
  free(): void;
/**
* @param {string} src
*/
  constructor(src: string);
/**
* @param {any} json
* @param {string} typename
* @returns {Uint8Array}
*/
  serialize(json: any, typename: string): Uint8Array;
/**
* @param {Uint8Array} bin
* @param {string} typename
* @returns {any}
*/
  deserialize(bin: Uint8Array, typename: string): any;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_wasm: () => void;
  readonly __wbg_smithjs_free: (a: number) => void;
  readonly smithjs_new: (a: number, b: number) => number;
  readonly smithjs_serialize: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly smithjs_deserialize: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
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
