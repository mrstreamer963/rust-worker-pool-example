// worker.js - простая версия без WASM импортов
// Вместо импорта WASM модуля, мы будем выполнять простую обработку задач

function processTask(payload) {
    // Простая обработка задачи без WASM
    const result = {
        id: payload.id || 0,
        result: `processed: ${payload.payload} (len=${payload.payload?.length || 0})`
    };
    return result;
}

self.onmessage = async (event) => {
    const { taskId, payload } = event.data;
    
    try {
        // Простая обработка задачи
        const result = processTask(payload);
        self.postMessage({ type: 'result', taskId, result });
    } catch (err) {
        self.postMessage({ type: 'error', taskId, error: err.toString() });
    }
};