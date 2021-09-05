import * as THREE from 'https://cdn.skypack.dev/three';

const all_geometries = [];
const material = new THREE.LineBasicMaterial({color: 0x2BC29F, linewidth: 3});

let t = 0;

export function init() {

    const renderer = new THREE.WebGLRenderer();
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);

    const camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 1, 500);

    camera.position.set(0, 0, 200);
    camera.lookAt(0, 0, 0);

    const scene = new THREE.Scene();


    // Since I'm describing a star whose center lies on the origin, it's
    // technically reflected across the y axis, so I really only have to
    // describe half of it

    // this is based off an orthographic camera about 100 euclidian units
    // from (0,0)
    
    const geos = get_star_geometries(10);

    for (const geo of geos) {
        scene.add(geo);
        all_geometries.push(geo);
    }

    function clamp(num, min, max) {
        return num <= min 
          ? min 
          : num >= max 
            ? max 
            : num;
      }

    function animate () {
        //A lot of HSV selectors put the hue values on a scale from 0 to 360 instead of 0 to 1.
        let h = sin_between(150.0 / 360.0, 120.0 / 360.0, t * 0.03);
        let s = 1;
        //t is multiplied by something to make change lightness either faster or slower than hue.
        let l = sin_between(0.55, 0.45, t * 0.03);
        material.color.setHSL(h, s, l);
        t += 1;

        //TODO use Object3D to rotate these instead. This is really slow

        // Calculate rotations in rad/sec: (speed * fps) 
        const speed = 0.02;

        requestAnimationFrame(animate);

        for (const geo of all_geometries) {
            geo.rotateY(speed);
        }

        renderer.render(scene, camera);
    };

    animate()
}


function sin_between(upper, lower, x) {
    let avg = (upper - lower) / 2;

    return avg * Math.sin(x) + (avg + lower);
}

function get_star_geometries(z) {
    const geos = [];

    const star_vertices = [
        [0, 70, z],
        [17.5, 26, z],
        [50, 16.4, z],
        [27.1, -20, z],
        [31.3, -70, z],
        [0, -48.3, z]
    ];

    const star_verticies_back = [
        [0, 70, -z],
        [17.5, 26, -z],
        [50, 16.4, -z],
        [27.1, -20, -z],
        [31.3, -70, -z],
        [0, -48.3, -z]
    ];

    const star_face_1 = create_line_from_vertex_data(star_vertices.concat(mirror_Y(star_vertices)));
    const star_face_2 = create_line_from_vertex_data(star_verticies_back.concat(mirror_Y(star_verticies_back)));

    geos.push(new THREE.Line(star_face_1, material));
    geos.push(new THREE.Line(star_face_2, material));

    //sv[0] to svb[0]
    //svb[0] to sv[1]
    //sv[1] to svb[1]
    //svb[1] to sv[2]

    let scaffolding_line = [];

    let i = 0;
    let flag = false;

    while (true) {        
        if (i == star_vertices.length - 1) {
            break;
        }

        if (!flag) {
            scaffolding_line.push(star_vertices[i]);
            scaffolding_line.push(star_verticies_back[i]);
        } else {
            scaffolding_line.push(star_vertices[i]);
            scaffolding_line.push(star_verticies_back[i + 1]);
            i++;
        }
        
        flag = !flag;
    }

    scaffolding_line.push(star_vertices[star_vertices.length - 1])

    geos.push(new THREE.Line(create_line_from_vertex_data(scaffolding_line.concat(mirror_Y(scaffolding_line))), material));

    return geos;
}

function mirror_Y(vecs) {
    let mirror = [];

    for (const vec of vecs) {
        mirror.push([-vec[0], vec[1], vec[2]]);
    }

    // If I don't reverse here, a line will be drawn from the bottom middle vertex of the star to the top middle vertex of the star.
    return mirror.reverse();
}

function create_line_from_vertex_data(verts) {
    let line_vecs = [];

    for (const vert of verts) {
        line_vecs.push(new THREE.Vector3(vert[0], vert[1], vert[2]));
    }

    return new THREE.BufferGeometry().setFromPoints(line_vecs);
}

function scaleXY(x, y, vec) {
    vec.x *= x;
    vec.y *= y;
}