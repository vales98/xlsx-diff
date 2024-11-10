import { diff as webDiff, load_workbook } from "./env/web";
import initWebWasm from './env/web'
let isWasmModuleInitialized = false;
const initWasmModule = async () => {
    if (isWasmModuleInitialized) {
        return;
    }
    await initWebWasm();
    isWasmModuleInitialized = true;
}
export const diff = async (dataA, dataB, withRawData = false) => {
    if (!dataA || !dataB) {
        throw new Error('Data cannot be empty');
    }
    if (!(dataA instanceof ArrayBuffer) || !(dataB instanceof ArrayBuffer)) {
        throw new Error('Data must be an ArrayBuffer');
    }
    const u8A = new Uint8Array(dataA);
    const u8B = new Uint8Array(dataB);
    await initWasmModule();
    const res = webDiff(u8A, u8B, withRawData);
    // start with Error means error
    if (res.startsWith('Error')) {
        throw new Error(res);
    }
    return JSON.parse(res);
}

export const loadWorkbook = async (data) => {
    if (!data) {
        throw new Error('Data cannot be empty');
    }
    if (!(data instanceof ArrayBuffer)) {
        throw new Error('Data must be an ArrayBuffer');
    }
    const u8Ary = new Uint8Array(data);
    await initWasmModule();
    const res = load_workbook(u8Ary);
    // start with Error means error
    if (res.startsWith('Error')) {
        throw new Error(res);
    }
    return JSON.parse(res);
}