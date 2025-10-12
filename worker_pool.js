// worker_pool.js

export class WorkerPool {
    constructor(workerUrl, size = 4) {
        if (size <= 0) throw new Error("Pool size must be > 0");

        this.workerUrl = workerUrl;
        this.size = size;
        this.workers = [];
        this.taskQueue = [];
        this.activeTasks = new Map();
        this.nextTaskId = 0;
        this.isDestroyed = false;

        for (let i = 0; i < size; i++) {
            this._createWorker();
        }
    }

    _createWorker() {
        const worker = new Worker(this.workerUrl, { type: 'module' });
        worker.onmessage = (e) => this._handleMessage(e);
        worker.onerror = (err) => {
            console.error('Worker error:', err);
        };
        this.workers.push({ worker, busy: false });
    }

    async runTask(payload) {
        if (this.isDestroyed) {
            throw new Error("WorkerPool is destroyed");
        }

        return new Promise((resolve, reject) => {
            this.taskQueue.push({ payload, resolve, reject });
            this._schedule();
        });
    }

    async runTasksBatch(payloads) {
        const promises = payloads.map(payload => this.runTask(payload));
        return await Promise.all(promises);
    }

    _schedule() {
        while (this.taskQueue.length > 0) {
            const freeWorker = this.workers.find(w => !w.busy);
            if (!freeWorker) break;

            const task = this.taskQueue.shift();
            const taskId = this.nextTaskId++;
            this.activeTasks.set(taskId, {
                resolve: task.resolve,
                reject: task.reject
            });
            this._assignTask(freeWorker, taskId, task.payload);
        }
    }

    _assignTask(worker, taskId, payload) {
        worker.busy = true;
        worker.worker.postMessage({ taskId, payload });
    }

    _handleMessage(event) {
        const { taskId, type, result, error } = event.data;
        const callbacks = this.activeTasks.get(taskId);
        if (!callbacks) {
            console.warn("Received result for unknown taskId:", taskId);
            return;
        }

        const worker = this.workers.find(w => w.worker === event.target);
        if (worker) worker.busy = false;

        this.activeTasks.delete(taskId);

        if (type === 'result') {
            callbacks.resolve(result);
        } else if (type === 'error') {
            callbacks.reject(new Error(error || 'Unknown worker error'));
        } else {
            callbacks.reject(new Error('Unknown message type from worker'));
        }

        this._schedule();
    }

    destroy() {
        if (this.isDestroyed) return;
        this.isDestroyed = true;

        for (const { reject } of this.taskQueue) {
            reject(new Error("WorkerPool destroyed"));
        }
        this.taskQueue = [];

        for (const { reject } of this.activeTasks.values()) {
            reject(new Error("WorkerPool destroyed"));
        }
        this.activeTasks.clear();

        this.workers.forEach(w => w.worker.terminate());
        this.workers = [];
    }
}