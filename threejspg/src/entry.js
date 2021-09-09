import { WebGLRenderer, PerspectiveCamera, Scene, Vector3 } from 'three';
import * as THREE from 'three';
import StarScene from './objects/Scene.js';
import { EffectComposer } from 'three/examples/jsm/postprocessing/EffectComposer.js';
import { UnrealBloomPass } from 'three/examples/jsm/postprocessing/UnrealBloomPass.js';
import { RenderPass } from 'three/examples/jsm/postprocessing/RenderPass.js';
import { SMAAPass } from 'three/examples/jsm/postprocessing/SMAAPass';

let effectComposer;

export function init() {
  const scene = new Scene();
  const camera = new PerspectiveCamera();

  let canvas = document.getElementById("login-canvas");
  let renderer = null;

  if (canvas) {
    console.log("Found canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    renderer = new WebGLRenderer({ canvas: canvas });
  } else {
    console.log("Didn't find canvas.");
    renderer = new WebGLRenderer();
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

  //Init postprocessing effects
  effectComposer = new EffectComposer(renderer);

  const renderPass = new RenderPass(scene, camera);
  effectComposer.addPass(renderPass);

  const bloomPass = new UnrealBloomPass(new THREE.Vector2(window.innerWidth * renderer.getPixelRatio(), window.innerHeight * renderer.getPixelRatio()), 1.5, 0, 1);
  effectComposer.addPass(bloomPass);

  const SMAApass = new SMAAPass(window.innerWidth * renderer.getPixelRatio(), window.innerHeight * renderer.getPixelRatio());
  effectComposer.addPass(SMAApass);

  // render loop
  const onAnimationFrameHandler = (timeStamp) => {
    renderer.render(scene, camera);
    seedScene.update && seedScene.update(timeStamp, renderer);
    window.requestAnimationFrame(onAnimationFrameHandler);
    effectComposer.render();
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
