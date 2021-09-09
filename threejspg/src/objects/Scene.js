import { Group, Matrix4, Scene } from 'three';
import BasicLights from './Lights.js';
import Star from './Star/Star.js';

export default class StarScene extends Group {
  static stars = [];

  constructor(frustum_check, scene) {
    super();
    this.frustum_check = frustum_check;
    this.scene = scene;

    const lights = new BasicLights();
    this.add(lights);

    for (const s of StarScene.stars) {
      this.add(s);
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
    for (const e of StarScene.stars) {
      e.playAnim();
    }

    let rand = Math.random();

    if (Math.random() > .8) {
      this.makeStar();
    }
  }


  randomNumber = function(min, max) {
    return Math.random() * (max - min) + min;
  }
}