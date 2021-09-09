import { Group, SpotLight, PointLight, AmbientLight, HemisphereLight, Color } from 'three';

export default class BasicLights extends Group {
  constructor(...args) {
    super(...args);

    const pointLight = new PointLight(0x4C1A88, 1, 0, 2)
    pointLight.position.set(-300, -100, 0)

    const pointLightTopRight = new PointLight(0xFFAF47, 1.2, 0, 2);
    pointLightTopRight.position.set(300, 100, 0);

    this.add(pointLight, pointLightTopRight);
  }
}
