import * as THREE from 'three';
import { MeshPhysicalMaterial, MeshStandardMaterial, Scene, Texture, Vector3 } from 'three';

export default class Star extends THREE.Object3D {
    // rotation_speed = speed at which star should rotate, in radians per frame.
    // move_speed = speed at which star should move, in units (meters?) per frame.
    constructor(rotation_speed, move_speed, func, parent, envMap) {
        super();
        this.rotation_speed = rotation_speed;
        this.move_speed = move_speed;
        this.name = "Star";
        this.func = func;
        this.parent = parent;
        this.envMap = envMap;
        this.t = 0;
        this.geometry = new THREE.ExtrudeGeometry(this.drawStar([
            [0, 4.5],
            [1.7, 1.6],
            [4.85, 0.96],
            [2.65, -1.5],
            [3.01, -4.75],
            [0, -3.4],
        ]), {
            steps: 2,
            depth: 1,
            bevelEnabled: true,
            bevelThickness: 1,
            bevelSize: 1,
            bevelOffset: 0,
            bevelSegments: 1
        });
        this.mat = new MeshPhysicalMaterial({
            clearcoat: 1,
            envMap: envMap,
        });
        this.mesh = new THREE.Mesh(this.geometry, this.mat);

        this.add(this.mesh);
    }

    randColor() {
        return 0x2bc29f;
    }

    drawStar(verts) {
        for (const e of this.reflectY(verts).reverse()) {
            verts.push(e);
        }

        const shape = new THREE.Shape();

        for (const [i, e] of verts.entries()) {
            if (i == 0) {
                shape.moveTo(e[0], e[1]);
                continue;
            }

            shape.lineTo(e[0], e[1]);
        }

        return shape;
    }

    frustumCheck(pos) {
        return this.func(pos) == true;
    }

    ignoreFrustrumCheck() {
        return this.t < 1 * 1000;
    }

    playAnim() {
        let pos = new Vector3(this.position.x, this.position.y, this.position.z);
        this.translateX(-this.move_speed);
        this.translateY(-this.move_speed);
        this.mesh.rotateY(this.rotation_speed);

        if (!this.ignoreFrustrumCheck() && !this.frustumCheck(pos.add(new Vector3(0, 10, 0)))) {
            this.parent.removeStar(this);
            return;
        }

        this.t++;
    }

    setStartingPosition() {
        function randomNumber(min, max) {
            return Math.random() * (max - min) + min;
        }

        this.position.setX(randomNumber(0, 300));
        this.position.setY(randomNumber(150, 200));
        this.position.setZ(randomNumber(-200, 50));
    }

    reflectY(verts) {
        let ref_verts = [];

        for (const vert of verts) {
            ref_verts.push([-vert[0], vert[1]]);
        }

        return ref_verts;
    }
}
