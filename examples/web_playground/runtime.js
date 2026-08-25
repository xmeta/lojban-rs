(() => {
  const mode = document.querySelector('meta[name="lojban-runtime"]')?.content || 'server';
  let wasmPromise = null;

  async function loadWasm() {
    if (!wasmPromise) {
      wasmPromise = import('./pkg/lojban_web.js')
        .then(async (module) => {
          await module.default();
          return module;
        })
        .catch((error) => {
          // 失敗したロードをキャッシュせず、次回呼び出しで再試行できるようにする。
          wasmPromise = null;
          throw error;
        });
    }
    return wasmPromise;
  }

  async function post(path, text) {
    const response = await fetch(`./api/${path}`, {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain; charset=utf-8' },
      body: text,
    });
    if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
    return response.json();
  }

  window.lojbanTransport = {
    mode,
    async parse(text) {
      if (mode === 'wasm') {
        const wasm = await loadWasm();
        return JSON.parse(wasm.parse_text(text));
      }
      return post('parse', text);
    },

    async regression(text) {
      if (mode === 'wasm') {
        const wasm = await loadWasm();
        return JSON.parse(wasm.regression_text(text));
      }
      return post('regression', text);
    },

    timingLabel(ms) {
      return mode === 'wasm'
        ? `${ms.toFixed(1)} ms browser`
        : `${ms.toFixed(1)} ms round trip`;
    },
  };
})();
