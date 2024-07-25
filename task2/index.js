// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg";

async function loadWasmModule() {
  const response = await fetch('./pkg/task2.js');
  const buffer = await response.arrayBuffer();
  const wasmModule = await WebAssembly.instantiate(buffer);
  return wasmModule.instance;
}

// Function to be called when the button is clicked
async function onButtonClick() {
  console.log("Test");
  const wasmInstance = await loadWasmModule();
  wasmInstance.exports.wasmFunction();
}

document.getElementById('wasmButton').addEventListener('click', onButtonClick);