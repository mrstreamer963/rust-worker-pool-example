// worker.js
import init, { run_task } from './pkg-web/worker_pool_demo.js';

let initialized = false;

async function ensureInit() {
    if (!initialized) {
        await init();
        initialized = true;
    }
}

self.onmessage = async (event) => {
    await ensureInit();

    const { taskId, payload } = event.data;

    try {
        const result = await run_task(payload);
        self.postMessage({ type: 'result', taskId, result });
    } catch (err) {
        self.postMessage({ type: 'error', taskId, error: err.toString() });
    }
};