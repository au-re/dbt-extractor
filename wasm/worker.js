import init, { extract_from_source_js } from './pkg/dbt_extractor_wasm.js';

self.onmessage = async (e) => {
  await init();
  const result = extract_from_source_js(e.data);
  self.postMessage(result);
};
