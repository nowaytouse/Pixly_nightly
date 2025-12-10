const path = require('path');
const fs = require('fs');


class WebpDecoder {
    constructor() {
        this.decoder = null;
    }


    async decode(src) {


        return new Promise((resolve, reject) => {
            // Read the webp file
            const buffer = fs.readFileSync(src);
            const arrayBuffer = new Uint8Array(buffer).buffer;

            // Create worker for webp decoding
            const workerPath = path.join(__dirname, 'webp-wasm.js');
            const worker = new Worker(workerPath);

            // Use transferable objects for better performance
            // This transfers ownership of the buffer to the worker, avoiding copy
            worker.postMessage({ image: arrayBuffer, hasAlpha: true }, [arrayBuffer]);

            worker.addEventListener('message', async (e) => {

                try {
                    // Handle error response from worker
                    if (e.data.error) {
                        worker.terminate();
                        const error = new Error(e.data.error.message);
                        error.stack = e.data.error.stack;
                        console.error('Worker reported error:', error);
                        reject(error);
                        return;
                    }

                    if (e.data.imgData) {
                        const imgData = e.data.imgData;

                        worker.terminate();

                        console.log('Successfully decoded WebP using WASM');

                        resolve(imgData);
                    }
                } catch (error) {
                    worker.terminate();
                    console.error('Error processing WebP image with WASM:', error);
                    reject(error);
                }

            });

            // Handle worker errors
            worker.addEventListener('error', (error) => {
                worker.terminate();
                console.error('WASM Worker error:', error);
                reject(error);
            });

            // Add timeout to prevent hanging
            const timeout = setTimeout(() => {
                worker.terminate();
                reject(new Error('WASM WebP decoding timeout (30s)'));
            }, 30000);

            // Clear timeout on success
            worker.addEventListener('message', () => clearTimeout(timeout), { once: true });
        });


    }
}

module.exports = new WebpDecoder();