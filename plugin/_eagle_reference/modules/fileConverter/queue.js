class Queue {
    constructor(concurrency = 8) {
        this.concurrency = concurrency;
        this.running = 0;
        this.queue = [];
        this.onComplete = null;
        this.cancelled = false;
        this.runningTasks = new Set();
        this.runningProcesses = new Map(); // Map<taskPromise, killProcess>
    }

    push(task) {
        if (!this.cancelled) {
            this.queue.push(task);
            this.next();
        }
    }

    next() {
        while (this.running < this.concurrency && this.queue.length && !this.cancelled) {
            const task = this.queue.shift();
            this.running++;
            
            const taskResult = task();
            let taskPromise;
            let killProcess = null;

            // 檢查任務是否返回可取消的結果
            if (taskResult && typeof taskResult.then === 'function') {
                // 標準 Promise
                taskPromise = taskResult;
            } else if (taskResult && taskResult.promise && typeof taskResult.promise.then === 'function') {
                // 可取消的結果：{promise, killProcess}
                taskPromise = taskResult.promise;
                killProcess = taskResult.killProcess;
            } else {
                // 其他情況，包裝為 Promise
                taskPromise = Promise.resolve(taskResult);
            }

            this.runningTasks.add(taskPromise);
            if (killProcess) {
                this.runningProcesses.set(taskPromise, killProcess);
            }
            
            taskPromise.finally(() => {
                this.running--;
                this.runningTasks.delete(taskPromise);
                this.runningProcesses.delete(taskPromise);
                
                if (!this.cancelled) {
                    this.next();
                }

                if (this.running === 0 && this.queue.length === 0 && this.onComplete) {
                    this.onComplete();
                }
            });
        }
    }

    onAllComplete(callback) {
        this.onComplete = callback;
    }
    
    cancel() {
        this.cancelled = true;
        const pendingTasks = this.queue.length;
        this.queue = [];
        
        // 終止所有執行中的進程
        let killedProcesses = 0;
        for (const [taskPromise, killProcess] of this.runningProcesses) {
            try {
                killProcess();
                killedProcesses++;
            } catch (error) {
                console.warn('[Queue] Failed to kill process:', error.message);
            }
        }
        
        console.log(`[Queue] Cancelled. Cleared ${pendingTasks} pending tasks, killed ${killedProcesses} running processes.`);
    }
    
    reset() {
        this.cancelled = false;
        this.queue = [];
        this.running = 0;
        this.runningTasks.clear();
        this.runningProcesses.clear();
    }
}

module.exports = Queue;