/**
 * Pixly AI Client
 * Communicates with Python AI HTTP Service
 */

class PixlyAIClient {
    constructor(baseURL = 'http://localhost:50052') {
        this.baseURL = baseURL;
        this.timeout = 30000; // 30 seconds
    }

    /**
     * Check AI service health
     */
    async checkHealth() {
        try {
            const response = await fetch(`${this.baseURL}/api/v1/health`, {
                method: 'GET',
                timeout: 5000
            });
            
            if (!response.ok) {
                throw new Error(`Health check failed: ${response.status}`);
            }
            
            return await response.json();
        } catch (error) {
            console.error('AI service health check failed:', error);
            throw new Error('AI service unavailable. Please ensure the Python AI service is running.');
        }
    }

    /**
     * Predict image optimization parameters
     */
    async predictImage(imagePath, mode = 'balanced', options = {}) {
        try {
            const response = await fetch(`${this.baseURL}/api/v1/predict`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    image_path: imagePath,
                    target_format: options.expected_format || 'auto',
                    mode: mode,
                    options: {
                        enable_format_recommendation: true,
                        enable_preprocessing: true,
                        enable_magika: options.enable_magika !== false
                    }
                }),
                timeout: this.timeout
            });

            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Image prediction failed');
            }

            return await response.json();
        } catch (error) {
            console.error('Image prediction error:', error);
            throw error;
        }
    }

    /**
     * Predict video optimization parameters
     */
    async predictVideo(videoPath, mode = 'balanced', options = {}) {
        try {
            const response = await fetch(`${this.baseURL}/api/v1/predict/video`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    video_path: videoPath,
                    mode: mode,
                    options: {
                        enable_format_recommendation: true,
                        enable_preprocessing: true,
                        aggressive_mode: options.aggressive_mode || false
                    }
                }),
                timeout: this.timeout
            });

            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Video prediction failed');
            }

            return await response.json();
        } catch (error) {
            console.error('Video prediction error:', error);
            throw error;
        }
    }

    /**
     * Predict audio optimization parameters
     */
    async predictAudio(audioPath, mode = 'balanced', options = {}) {
        try {
            const response = await fetch(`${this.baseURL}/api/v1/predict/audio`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    audio_path: audioPath,
                    mode: mode,
                    options: {
                        enable_format_recommendation: true,
                        enable_preprocessing: true,
                        aggressive_mode: options.aggressive_mode || false,
                        enable_magika: options.enable_magika !== false
                    }
                }),
                timeout: this.timeout
            });

            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Audio prediction failed');
            }

            return await response.json();
        } catch (error) {
            console.error('Audio prediction error:', error);
            throw error;
        }
    }

    /**
     * Auto-detect media type and predict
     */
    async predictAuto(filePath, mode = 'balanced', options = {}) {
        const ext = filePath.split('.').pop().toLowerCase();
        
        // Image formats
        const imageFormats = ['jpg', 'jpeg', 'png', 'webp', 'avif', 'jxl', 'gif', 'bmp', 'tiff'];
        if (imageFormats.includes(ext)) {
            return await this.predictImage(filePath, mode, options);
        }
        
        // Video formats
        const videoFormats = ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'm4v'];
        if (videoFormats.includes(ext)) {
            return await this.predictVideo(filePath, mode, options);
        }
        
        // Audio formats
        const audioFormats = ['mp3', 'aac', 'm4a', 'opus', 'ogg', 'flac', 'wav'];
        if (audioFormats.includes(ext)) {
            return await this.predictAudio(filePath, mode, options);
        }
        
        throw new Error(`Unsupported file format: ${ext}`);
    }
}

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PixlyAIClient;
}
