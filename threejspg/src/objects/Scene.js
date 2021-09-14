import * as THREE from 'three';
import Star from './Star/Star.js';

export default class StarScene extends THREE.Group {
  static stars = [];

  constructor(frustum_check, scene) {
    super();
    this.frustum_check = frustum_check;
    this.scene = scene;
    this.t = 0;
    this.makePointStars();
  }

  randomNumber(min, max) {
    return Math.random() * (max - min) + min;
  }

  makePointStars() {
    for (let z = -1500; z < 1000; z += 13) {
      const lilSphere = new THREE.SphereGeometry(1.3, 32, 32);
      const mat = new THREE.MeshBasicMaterial();
      
      mat.color.setRGB(Math.random(), Math.random(), Math.random());
      this.mat = mat;
      const mesh = new THREE.Mesh(lilSphere, mat);

      mesh.position.x = Math.random() * 1000 - 500;
      mesh.position.y = Math.random() * 1000 - 500;

      mesh.position.z = z;
      this.add(mesh);
      StarScene.stars.push(mesh);
    }
  }

  update(timestamp, renderer) {
    this.t += 0.001;

    for (let i = 0; i < StarScene.stars.length; i++) {
      let star = StarScene.stars[i];
      star.position.z += i / 25;

      if (star.position.z > 1000) {
        star.position.z -= 2000;
      }
    }
  }


  randomNumber(min, max) {
    return Math.random() * (max - min) + min;
  }
}