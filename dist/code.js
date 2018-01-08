if (typeof require === 'function') // test for nodejs environment
{
  var THREE = require('three');
}

// Global variables. Used mostly in rust code
var mouseX = 0, mouseY = 0
var frustumSize = 1000;
var aspect = window.innerWidth / window.innerHeight;

var scene = new THREE.Scene();
var camera = new THREE.OrthographicCamera(frustumSize * aspect / 2, frustumSize * aspect / -2, frustumSize / -2, frustumSize / 2, 0.0001, 5000);
var renderer = new THREE.WebGLRenderer({ antialias: true });
var gameObjects = {};
var materials = {};
var geometry = {};
var module;

function prepareGeometry() {
  // Default
  geometry[0] = new THREE.BoxGeometry(50, 50, 50);
  // Player
  geometry[1] = new THREE.BoxGeometry(150, 30, 50);
  // Ball
  geometry[2] = new THREE.SphereGeometry(25, 16, 16);

  // Side walls
  geometry[3] = new THREE.BoxGeometry(50, 1000, 50);
  // Top and down walls
  geometry[4] = new THREE.BoxGeometry(1000, 50, 50);
}

function prepareMaterials() {
  // Default
  materials[0] = new THREE.MeshStandardMaterial({
    roughness: 0.7,
    color: 0x3F3F3F,
    metalness: 0.2
  });
  // Player
  materials[1] = new THREE.MeshStandardMaterial({
    emissive: 0x000000,
    roughness: 1,
    color: 0x000000,
    metalness: 1.0
  });
  // Ball
  materials[2] = new THREE.MeshStandardMaterial({
    emissive: 0x000000,
    color: 0x005050,
    metalness: 1.0
  });
  // Walls
  materials[3] = new THREE.MeshStandardMaterial({
    emissive: 0x000000,
    roughness: 1,
    color: 0x000000,
    metalness: 1.0
  });
}


mainWithGlTest();

function mainWithGlTest() {
  if (Detector.webgl) {
    Rust.wasm_test.then(function (wasm_test) {
      module = wasm_test;
      main();
    });
  } else {
    document.body.appendChild(Detector.getWebGLErrorMessage());
  }
}

function main() {
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  window.addEventListener('resize', onWindowResize, false);
  document.addEventListener('mousemove', onDocumentMouseMove, false);
  document.addEventListener('touchstart', onDocumentTouchStart, false);
  document.addEventListener('touchmove', onDocumentTouchMove, false);

  document.body.appendChild(renderer.domElement);

  prepareGeometry()
  prepareMaterials();

  camera.position.z = 400;
  camera.lookAt(scene.position);
  camera.rotation.z = Math.PI;

  var gridHelper = new THREE.GridHelper(1000, 20);
  gridHelper.rotation.x = Math.PI / 2.0;
  //scene.add(gridHelper);
  //scene.add(new THREE.AxesHelper(100));

  var sphere = new THREE.SphereGeometry(50.0, 16, 8);

  var light1 = new THREE.PointLight(0xFFFFFF, 15, 5000, 2.0);
  light1.add(new THREE.Mesh(sphere, new THREE.MeshBasicMaterial({ color: 0xffFFFF })));
  light1.position.z = 1000;
  scene.add(light1);

  var floor = new THREE.BoxGeometry(1000, 1000, 0.1);
  var floorMat = new THREE.MeshStandardMaterial({
    emissive: 0x808080,
    roughness: 1,
    color: 0x202020,
    metalness: 1.0
  });
  var floorMesh = new THREE.Mesh(floor, floorMat);
  floorMesh.position.z = -1;
  scene.add(floorMesh);

  module.init();
  module.on_sceen_size_changed(window.innerWidth, window.innerHeight);
}

function onWindowResize() {
  var aspect = window.innerWidth / window.innerHeight;
  camera.left = frustumSize * aspect / 2;
  camera.right = - frustumSize * aspect / 2;
  camera.top = - frustumSize / 2;
  camera.bottom = frustumSize / 2;

  camera.updateProjectionMatrix();
  renderer.setSize(window.innerWidth, window.innerHeight);
  module.on_sceen_size_changed(window.innerWidth, window.innerHeight);
}

function onDocumentMouseMove(event) {
  mouseX = event.clientX;
  mouseY = event.clientY;
  module.on_mouse_move(mouseX, mouseY);
}

function onDocumentTouchStart(event) {
  if (event.touches.length > 1) {
    event.preventDefault();
    mouseX = event.touches[0].pageX;
    mouseY = event.touches[0].pageY;
    module.on_mouse_move(mouseX, mouseY);
  }
}

function onDocumentTouchMove(event) {
  if (event.touches.length == 1) {
    event.preventDefault();
    mouseX = event.touches[0].pageX;
    mouseY = event.touches[0].pageY;
    module.on_mouse_move(mouseX, mouseY);
  }
}