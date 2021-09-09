import { Group, Matrix4, Scene } from 'three';
import * as THREE from 'three';
import Star from './Star/Star.js';

export default class StarScene extends Group {
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

  HSVtoRGB(h, s, v) {
    var r, g, b, i, f, p, q, t;
    if (arguments.length === 1) {
      s = h.s, v = h.v, h = h.h;
    }
    i = Math.floor(h * 6);
    f = h * 6 - i;
    p = v * (1 - s);
    q = v * (1 - f * s);
    t = v * (1 - (1 - f) * s);
    switch (i % 6) {
      case 0: r = v, g = t, b = p; break;
      case 1: r = q, g = v, b = p; break;
      case 2: r = p, g = v, b = t; break;
      case 3: r = p, g = q, b = v; break;
      case 4: r = t, g = p, b = v; break;
      case 5: r = v, g = p, b = q; break;
    }
    return {
      r: Math.round(r * 255),
      g: Math.round(g * 255),
      b: Math.round(b * 255)
    };
  }

  rainbow(p) {
    var rgb = this.HSVtoRGB(p / 100.0 * 0.85, 1.0, 1.0);
    return [rgb.r, rgb.g, rgb.b];
  }

  updateMatColor(star) {
    let rainbow = this.rainbow(this.t);
    star.material.color.setRGB(rainbow[0], rainbow[1], rainbow[2]);
  }

  makePointStars() {
    for (let z = -1000; z < 1000; z += 10) {
      const lilSphere = new THREE.SphereGeometry(this.randomNumber(0.5, 1.0), 32, 32);
      const mat = new THREE.MeshBasicMaterial({ color: this.rainbow(this.t) });
      this.mat = mat;
      const mesh = new THREE.Mesh(lilSphere, mat);

      mesh.position.x = Math.random() * 1000 - 500;
      mesh.position.y = Math.random() * 1000 - 500;

      mesh.position.z = z;
      this.add(mesh);
      StarScene.stars.push(mesh);
    }
  }

  makeInitialStars() {
    for (let i = 0; i < 200; i++) {
      this.makeStar();
    }
  }

  makeStar() {
    const star = new Star(this.randomNumber(0.05, 0.1), this.randomNumber(0.4, 1.8), this.frustum_check, this, this.scene.background);
    star.setStartingPosition();
    StarScene.stars.push(star);
    this.add(star);
  }

  removeStar(star) {
    star.geometry.dispose();
    star.mat.dispose();
    this.remove(star);
    StarScene.stars = StarScene.stars.filter(e => e != star);
  }

  update(timestamp, renderer) {
    this.t += 0.001;

    for (let i = 0; i < StarScene.stars.length; i++) {
      let star = StarScene.stars[i];
      star.position.z += i / 10;
      this.updateMatColor(star);

      if (star.position.z > 1000) {
        star.position.z -= 2000;
      }
    }
  }


  randomNumber = function (min, max) {
    return Math.random() * (max - min) + min;
  }
}