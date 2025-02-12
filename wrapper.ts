// Import WASM
import * as wasm from "./dist/web/jazz_crypto_rs_bg.wasm";

// Export glue code
export * from "./dist/web/jazz_crypto_rs_bg.js";

// Initialize WASM
import { __wbg_set_wasm } from "./dist/web/jazz_crypto_rs_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
