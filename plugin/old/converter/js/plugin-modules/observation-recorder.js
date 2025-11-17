/**
 * 📊 观测数据记录器 (Observation Recorder)
 * 用于PPO强化学习训练
 * 
 * 功能:
 * - 记录每次转换的参数和结果
 * - 计算奖励函数
 * - 发送观测数据到GO核心
 * - 管理本地观测缓存
 * 
 * @version 5.4.0
 * @date 2025-11-03
 */

(function() {
    'use strict';

    // 观测数据记录器类
    class ObservationRecorder {
        constructor() {
            this.observations = [];
            this.maxLocalCache = 100;
            this.enabled = false;
            this.apiEndpoint = 'http://localhost:50052/api/v1/observations';
        }

        /**
         * 启用观测记录
         */
        enable() {
            this.enabled = true;
            Logger.info('[Observation] 📊 观测记录已启用');
        }

        /**
         * 禁用观测记录
         */
        disable() {
            this.enabled = false;
        }

        /**
         * 记录一次转换观测
         * @param {Object} observation - 观测数据
         */
        async record(observation) {
            if (!this.enabled) return;

            try {
                // 验证观测数据
                if (!this._validateObservation(observation)) {
                    Logger.warn('[Observation] ⚠️ Invalid observation data, skipped');
                    return;
                }

                // 计算奖励
                const reward = this._calculateReward(observation);
                observation.reward = reward;
                observation.timestamp = new Date().toISOString();

                // 添加到本地缓存
                this.observations.push(observation);
                if (this.observations.length > this.maxLocalCache) {
                    this.observations.shift(); // 移除最老的观测
                }

                Logger.debug(
                    `[Observation] 📝 Recorded | ` +
                    `File: ${observation.file_name} | ` +
                    `Q=${observation.params.quality} | ` +
                    `SSIM=${observation.ssim.toFixed(4)} | ` +
                    `Size: ${(observation.output_size / 1024).toFixed(1)}KB | ` +
                    `Reward=${reward.toFixed(4)}`
                );

                // 发送到GO核心
                await this._sendToServer(observation);

            } catch (error) {
                Logger.error('[Observation] ❌ Record failed:', error);
            }
        }

        /**
         * 验证观测数据
         */
        _validateObservation(obs) {
            return (
                obs.file_name &&
                obs.tool &&
                obs.optimize_mode &&
                obs.params &&
                typeof obs.params.quality === 'number' &&
                typeof obs.ssim === 'number' &&
                typeof obs.input_size === 'number' &&
                typeof obs.output_size === 'number' &&
                typeof obs.conversion_time === 'number'
            );
        }

        /**
         * 计算奖励函数
         * 
         * 奖励 = w1 * SSIM + w2 * 压缩率 + w3 * 速度
         * 
         * SSIM: 越高越好 (0-1)
         * 压缩率: 越小越好，但需要归一化
         * 速度: 越快越好，但需要归一化
         */
        _calculateReward(obs) {
            // 权重（可从种子库读取）
            const w_ssim = 0.5;
            const w_size = 0.3;
            const w_speed = 0.2;

            // SSIM分量 (0-1, 越高越好)
            const ssim_score = Math.max(0, Math.min(1, obs.ssim));

            // 压缩率分量 (归一化到0-1, 越小越好)
            const compression_ratio = obs.output_size / obs.input_size;
            const size_score = Math.max(0, 1 - compression_ratio); // 反转，越小压缩率越高分

            // 速度分量 (归一化到0-1, 越快越好)
            // 假设5秒以内都是快的
            const speed_score = Math.max(0, 1 - (obs.conversion_time / 5000));

            // 综合奖励
            const reward = (
                w_ssim * ssim_score +
                w_size * size_score +
                w_speed * speed_score
            );

            return reward;
        }

        /**
         * 发送观测数据到GO核心
         */
        async _sendToServer(observation) {
            try {
                const response = await fetch(this.apiEndpoint, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(observation)
                });

                if (!response.ok) {
                    throw new Error(`Server responded with ${response.status}`);
                }

                const result = await response.json();
                Logger.debug(
                    `[Observation] ✅ Sent to server | ` +
                    `Total observations: ${result.total_observations || 'N/A'}`
                );

            } catch (error) {
                // 服务器不可用时不报错，只记录日志
                Logger.debug('[Observation] ⚠️ Server unavailable, observation cached locally');
            }
        }

        /**
         * 获取本地缓存的观测数据
         */
        getLocalObservations() {
            return this.observations;
        }

        /**
         * 清空本地缓存
         */
        clearLocalCache() {
            const count = this.observations.length;
            this.observations = [];
            Logger.info(`[Observation] 🗑️ Cleared ${count} local observations`);
        }

        /**
         * 获取统计信息
         */
        getStats() {
            if (this.observations.length === 0) {
                return {
                    count: 0,
                    avg_ssim: 0,
                    avg_reward: 0,
                    avg_compression: 0
                };
            }

            const stats = {
                count: this.observations.length,
                avg_ssim: 0,
                avg_reward: 0,
                avg_compression: 0
            };

            this.observations.forEach(obs => {
                stats.avg_ssim += obs.ssim;
                stats.avg_reward += obs.reward;
                stats.avg_compression += (obs.output_size / obs.input_size);
            });

            stats.avg_ssim /= stats.count;
            stats.avg_reward /= stats.count;
            stats.avg_compression /= stats.count;

            return stats;
        }
    }

    // 创建全局实例
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ObservationRecorder = new ObservationRecorder();

    const Logger = window.PIXLY?.Logger || console;
    Logger.info('Observation', '📊 Observation Recorder v5.4.0 initialized');
})();
