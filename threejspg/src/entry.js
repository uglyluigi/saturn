import { WebGLRenderer, PerspectiveCamera, Scene, Vector3 } from 'three';
import * as THREE from 'three';
import StarScene from './objects/Scene.js';

export function init() {
  const scene = new Scene();
  const camera = new PerspectiveCamera();

  let canvas = document.getElementById("login-canvas");
  let renderer = null;

  if (canvas) {
    console.log("Found canvas");
    renderer = new WebGLRenderer({ antialias: true, canvas: canvas });
  } else {
    console.log("Didn't find canvas.");
    renderer = new WebGLRenderer({ antialias: true });
    document.body.appendChild(renderer.domElement);
  }
  const seedScene = new StarScene(function (pos) {
    camera.updateMatrix();
    camera.updateMatrixWorld();
    var frustum = new THREE.Frustum();
    frustum.setFromProjectionMatrix(new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse));

    return frustum.containsPoint(pos);
  }, scene);

  scene.add(seedScene);

  // camera
  camera.position.set(45, window.innerWidth / window.innerHeight, 1, 1000);
  camera.position.z = 5;

  // renderer
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.setClearColor(0x000000, 1);

  // render loop
  const onAnimationFrameHandler = (timeStamp) => {
    renderer.render(scene, camera);
    seedScene.update && seedScene.update(timeStamp, renderer);
    window.requestAnimationFrame(onAnimationFrameHandler);
  }

  window.requestAnimationFrame(onAnimationFrameHandler);

  // resize
  const windowResizeHanlder = () => {
    const { innerHeight, innerWidth } = window;
    renderer.setSize(innerWidth, innerHeight);
    camera.aspect = innerWidth / innerHeight;
    camera.updateProjectionMatrix();
  };
  windowResizeHanlder();
  window.addEventListener('resize', windowResizeHanlder);

  // dom
  document.body.style.margin = 0;
}

init();
// scene
