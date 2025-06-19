const worker = new Worker('../worker.js', { type: 'module' });

const input = document.getElementById('input');
const output = document.getElementById('output');

function run() {
  worker.postMessage(input.value);
}

worker.onmessage = (e) => {
  output.textContent = JSON.stringify(e.data, null, 2);
};

input.addEventListener('input', run);
run();

