module.exports = async (src, plugin) => {
    return new Promise(async (resolve, reject) => {
        const fs = require('fs');
        const script = document.createElement('script');
        script.src = `${require("path").normalize(plugin.path)}\\libheif.js`;
        script.onload = async () => {
            try {
                // https://www.npmjs.com/package/libheif-js 1.18.2
                const libheifCore = await libheif();
                
                // TODO: when wasm is loaded
                await new Promise(resolve => setTimeout(resolve, 50));
                const decoder = new libheifCore.HeifDecoder();
                
                const buffer = fs.readFileSync(src);
                const arrayBuffer = new Uint8Array(buffer).buffer;
                
                const imageData = decoder.decode(arrayBuffer);
                const image = imageData[0];

                // create canvas and image data
                const canvas = document.createElement('canvas');
                const [w, h] = [image.get_width(), image.get_height()];
                canvas.width = w;
                canvas.height = h;

                const ctx = canvas.getContext('2d');
                const image_data = ctx.createImageData(canvas.width, canvas.height);
                var data = image_data.data;

                for (var i = 0; i < w * h; i++) {
                    data[i * 4 + 3] = 255;
                }

                await new Promise((resolveDisplay, rejectDisplay) => {
                    image.display(image_data, (display_image_data) => {
                        try {
                            ctx.putImageData(display_image_data, 0, 0);
                            resolveDisplay();
                        }
                        catch (e) {
                            rejectDisplay(e);
                        }
                    });
                });

                resolve({
                    canvas: canvas,
                    width: w,
                    height: h
                });
            } catch (e) {
                console.log(e);
                try {
                    console.log('This image may not be in HEIC format, trying to load as JPG/PNG instead...');
                    const image = await new Promise((resolve, reject) => {
                        const img = new Image();
                        img.src = src;
                        img.onload = () => resolve(img);
                        img.onerror = reject;
                    });

                    const canvas = document.createElement('canvas');
                    canvas.width = image.width;
                    canvas.height = image.height;
                    const ctx = canvas.getContext('2d');
                    ctx.drawImage(image, 0, 0);

                    if (image.width && image.height) {
                        console.log('Successfully loaded as JPG/PNG');
                        resolve({
                            canvas: canvas,
                            width: image.width,
                            height: image.height
                        });
                    }
                    else {
                        console.log('Failed to load as JPG/PNG');
                        reject('Invalid image');
                    }
                }
                catch (e) {
                    console.log(e);
                    reject(e);
                }
            }
        };

        script.onerror = reject;
        document.body.appendChild(script);
    });
};