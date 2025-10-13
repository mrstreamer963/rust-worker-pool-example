// worker.js - Web Worker для обработки задач с WASM
import init, { run_task } from './pkg/worker_pool_demo.js';

let wasmInitialized = false;

async function initializeWasm() {
    if (!wasmInitialized) {
        await init();
        wasmInitialized = true;
    }
}

async function processTask(payload) {
    await initializeWasm();
    
    // Используем WASM функцию для обработки задачи
    const result = await run_task(payload);
    return result;
}

self.onmessage = async (event) => {
    const { taskId, payload } = event.data;
    
    try {
        // Обработка задачи через WASM
        const result = await processTask(payload);
        self.postMessage({ type: 'result', taskId, result });
    } catch (err) {
        console.error('Worker error:', err);
        self.postMessage({ type: 'error', taskId, error: err.toString() });
    }
};