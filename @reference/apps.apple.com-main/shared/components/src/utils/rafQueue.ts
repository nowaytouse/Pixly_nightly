/**
 * @name RequestAnimationFrameLimiter
 * @description
 * allows for multiple callbacks to be called
 * within a single RAF function.
 * It also spreads long running tasks across multiple
 * microtask to help keep the main thread free for user interactions
 *
 */
export class RequestAnimationFrameLimiter {
    private queue: Array<(timestamp?: number) => void>;
    private RAF_FN_LIMIT_MS: number;
    private requestId: number | null;
    constructor() {
        this.queue = [];
        // ideal limit for scroll based animations: https://developers.google.com/web/fundamentals/performance/rendering/optimize-javascript-execution#reduce_complexity_or_use_web_workers
        this.RAF_FN_LIMIT_MS = 3;
        this.requestId = null;
    }

    private flush(): void {
        this.requestId =
            this.queue.length === 0
                ? null
                : window.requestAnimationFrame((timestamp) => {
                      const start = window.performance.now();
                      let ellapsedTime = 0;
                      const { RAF_FN_LIMIT_MS } = this;
                      let count = 0;

                      while (
                          count < this.queue.length &&
                          ellapsedTime < RAF_FN_LIMIT_MS
                      ) {
                          let item = this.queue[count];
                          if (item) {
                              item(timestamp);
                          }
                          const finishTime = window.performance.now();

                          count = count + 1;
                          ellapsedTime = finishTime - start;
                      }
                      const newQueue = this.queue.slice(count);

                      this.queue = newQueue;
                      this.flush();
                  });
    }
    public add(callback: () => void): void {
        this.queue.push(callback);
        if (this.requestId === null) {
            this.flush();
        }
    }
}

let raf: RequestAnimationFrameLimiter | ServerSafeRAFLimiter = null;

type ServerSafeRAFLimiter = {
    add: (callback: () => void) => void;
};

export const getRafQueue = () => {
    if (typeof window === 'undefined') {
        // SSR safe
        raf = {
            add: (callback: () => void) => callback(),
        };
    } else if (raf === null) {
        raf = new RequestAnimationFrameLimiter();
    }
    return raf;
};
