import { diff as webDiff } from "./env/web";
import initWebWasm from './env/web'
export const diff = async (dataA, dataB, withRawData = false) => {
    if (!dataA || !dataB) {
        throw new Error('Data cannot be empty');
    }
    if (!(dataA instanceof ArrayBuffer) || !(dataB instanceof ArrayBuffer)) {
        throw new Error('Data must be an ArrayBuffer');
    }
    // to unit8array
    const u8A = new Uint8Array(dataA);
    const u8B = new Uint8Array(dataB);
    await initWebWasm();
    const res = webDiff(u8A, u8B, withRawData);
    // start with Error means error
    if (res.startsWith('Error')) {
        throw new Error(res);
    }
    return JSON.parse(res);
}