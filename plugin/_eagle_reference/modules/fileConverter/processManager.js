// processManager.js
const { spawn } = require("child_process");

class ProcessManager {
    constructor() {
        // 用來存所有的 child_process
        // 格式: { id: number, process: ChildProcess, command: string, args: string[] }
        this.processes = new Map();
        this._nextId = 1;
    }

    /**
     * 建立一個 child_process
     * @param {string} command - 可執行檔 (例如 ffmpeg)
     * @param {string[]} args - 參數陣列
     * @param {object} options - spawn 的 options
     * @returns {object} { id, process }
     */
    spawn(command, args = [], options = {}) {
        const child = spawn(command, args, options);

        const id = this._nextId++;
        this.processes.set(id, { id, process: child, command, args });

        // 自動清理已結束的 process
        child.on("exit", () => {
            this.processes.delete(id);
        });

        return child;
    }

    /**
     * 根據 id 取得 process
     */
    getProcess(id) {
        return this.processes.get(id);
    }

    /**
     * 殺掉特定 id 的 process
     */
    killById(id) {
        const entry = this.processes.get(id);
        if (entry) {
            entry.process.kill("SIGKILL");
            this.processes.delete(id);
            return true;
        }
        return false;
    }

    /**
     * 殺掉所有正在執行的 process
     */
    killAll() {
        for (const { id, process } of this.processes.values()) {
            process.kill("SIGKILL");
            this.processes.delete(id);
        }
    }

    /**
     * 列出所有正在執行的 process
     */
    list() {
        return Array.from(this.processes.values()).map(p => ({
            id: p.id,
            command: p.command,
            args: p.args
        }));
    }
}

module.exports = new ProcessManager();
