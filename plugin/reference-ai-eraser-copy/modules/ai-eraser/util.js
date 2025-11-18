async function checkWebgpu() {
    // @ts-ignore
    if (!navigator.gpu) {
        return false;
    }
    // @ts-ignore
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
        return false;
    }
    return true;
}
const wasm = () => typeof WebAssembly === 'object' && typeof WebAssembly.instantiate === 'function';
const threads = () =>
    (async (e) => {
        try {
            return (
                typeof MessageChannel !== 'undefined' &&
                    new MessageChannel().port1.postMessage(new SharedArrayBuffer(1)),
                WebAssembly.validate(e)
            );
        } catch (e) {
            return !1;
        }
    })(
        new Uint8Array([
            0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 4, 1, 3, 1, 1, 10, 11, 1,
            9, 0, 65, 0, 254, 16, 2, 0, 26, 11
        ])
    );
const simd = async () =>
    WebAssembly.validate(
        new Uint8Array([
            0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 123, 3, 2, 1, 0, 10, 10, 1, 8, 0, 65, 0,
            253, 15, 253, 98, 11
        ])
    );

const getCapabilities = async () => {
    return {
        webgpu: await checkWebgpu(),
        wasm: wasm(),
        simd: await simd(),
        threads: await threads()
    };
};

module.exports = {
    getCapabilities
};
