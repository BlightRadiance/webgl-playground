const canvas = document.querySelector("#canvas");
const gl = canvas.getContext("webgl");

if (!gl) {
  alert("Unable to initialize WebGL. Your browser or machine may not support it.");
} else {
  prepareCanvas();
  renderFrame();
}

function prepareCanvas() {
  function resize() {
    canvas.width = window.innerWidth - 4;
    canvas.height = window.innerHeight - 4;
    renderFrame();
  }
  window.addEventListener('resize', resize, false); resize();
  return canvas;
}

function renderFrame() {
  gl.clearColor(0.0, 0.0, 0.0, 1.0);
  gl.clear(gl.COLOR_BUFFER_BIT);
}