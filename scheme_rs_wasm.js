import * as wasm from "./scheme_rs_wasm_bg.wasm";
export * from "./scheme_rs_wasm_bg.js";
import { __wbg_set_wasm } from "./scheme_rs_wasm_bg.js";
__wbg_set_wasm(wasm);