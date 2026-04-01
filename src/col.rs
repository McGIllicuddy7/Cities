use raylib::math::{BoundingBox, Quaternion, Ray, Rectangle, Vector2, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Collider2D {
    pub pos: Vector2, //center position
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub velocity: Vector2,
    pub mass: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Collider3D {
    pub pos: Vector3, //center position
    pub rotation: Quaternion,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub velocity: Vector3,
    pub mass: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ColData3D {
    pub pos: Vector3,
    pub normal: Vector3,
    pub dist: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct ColData2D {
    pub pos: Vector2,
    pub normal: Vector2,
    pub dist: f32,
}

impl Collider2D {
    pub fn as_vertices(&self) -> [Vector2; 4] {
        [
            self.pos + Vector2::new(-self.width / 2., -self.height / 2.).rotated(self.rotation),
            self.pos + Vector2::new(-self.width / 2., self.height / 2.).rotated(self.rotation),
            self.pos + Vector2::new(self.width / 2., -self.height / 2.).rotated(self.rotation),
            self.pos + Vector2::new(self.width / 2., self.height / 2.).rotated(self.rotation),
        ]
    }

    pub fn sap_vectors(&self) -> [Vector2; 12] {
        [
            Vector2::new(-self.width / 2., -self.height / 2.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(-self.width / 2., self.height / 2.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(self.width / 2., -self.height / 2.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(self.width / 2., self.height / 2.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(-self.width / 2., 0.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(0., -self.height / 2.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(self.width / 2., 0.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(0., self.height / 2.)
                .rotated(self.rotation)
                .normalized(),
            Vector2::new(1., 0.).rotated(self.rotation),
            Vector2::new(0., 1.).rotated(self.rotation),
            Vector2::new(-1., 0.).rotated(self.rotation),
            Vector2::new(0., 1.).rotated(self.rotation),
        ]
    }

    pub fn check_collision(&self, other: &Self) -> bool {
        let norms = [self.sap_vectors(), other.sap_vectors()];
        let sverts = self.as_vertices();
        let overts = other.as_vertices();
        for i in norms.iter().flatten() {
            let mut smin = sverts[0].dot(*i);
            let mut smax = smin;
            let mut omin = overts[0].dot(*i);
            let mut omax = omin;
            for j in 0..4 {
                let d = sverts[j].dot(*i);
                if d > smax {
                    smax = d;
                }
                if d < smin {
                    smin = d;
                }
            }
            for j in 0..4 {
                let d = overts[j].dot(*i);
                if d > omax {
                    omax = d;
                }
                if d < smin {
                    omin = d;
                }
            }
            if (smin < omin && smax < omin) || (smax > omax && smin > omax) {
                return false;
            }
        }
        true
    }

    pub fn normals(&self) -> [Vector2; 4] {
        [
            Vector2::new(-1., 0.).rotated(self.rotation),
            Vector2::new(1., 0.).rotated(self.rotation),
            Vector2::new(0., -1.).rotated(self.rotation),
            Vector2::new(0., 1.).rotated(self.rotation),
        ]
    }

    pub fn raycast(&self, pos: Vector2, dir: Vector2) -> Option<ColData2D> {
        let trans = -self.rotation;
        let rc = Rectangle::new(-self.width / 2., -self.height / 2., self.width, self.height);
        let end = (dir
            * (Vector2::distance_to(&pos, self.pos) + self.width * 4. + self.height * 4.))
            .rotated(trans);
        let start = Vector2::zero();
        if rc.check_collision_point_rec(pos - self.pos) {
            return Some(ColData2D {
                pos,
                normal: (-dir).normalized(),
                dist: 0.0,
            });
        } else {
            let segments = [
                (
                    Vector2::new(rc.x, rc.y),
                    Vector2::new(rc.x + rc.width, rc.y),
                ),
                (
                    Vector2::new(rc.x, rc.y),
                    Vector2::new(rc.x, rc.y + rc.height),
                ),
                (
                    Vector2::new(rc.x + rc.width, rc.y + rc.height),
                    Vector2::new(rc.x + rc.width, rc.y),
                ),
                (
                    Vector2::new(rc.x + rc.width, rc.y + rc.height),
                    Vector2::new(rc.x, rc.y + rc.height),
                ),
            ];
            for i in segments {
                if let Some(x) = raylib::check_collision_lines(start, end, i.0, i.1) {
                    let nm = Vector2::normalized(&(i.1 - i.0)).rotated(std::f32::consts::PI / 2.0);
                    let dx = ((i.1 + i.0) / 2.0).normalized().dot(nm);
                    let nm = if dx > 0. { nm } else { -nm };
                    return Some(ColData2D {
                        dist: x.length(),
                        pos: (x).rotated(self.rotation) + self.pos,
                        normal: nm.rotated(self.rotation),
                    });
                }
            }
            None
        }
    }

    pub fn point_distance_to(&self, p: Vector2) -> f32 {
        let mut out = 1000000.0;
        let v = self.as_vertices();
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                let p1 = v[i];
                let p2 = v[j];
                let dist = point_distance_to_line_2d(p, p1, p2);
                if dist < out {
                    out = dist;
                }
            }
        }
        out
    }
}

impl Collider3D {
    pub fn as_vertices(&self) -> [Vector3; 8] {
        [
            self.pos
                + Vector3::new(-self.width / 2., -self.height / 2., -self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(-self.width / 2., self.height / 2., -self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(self.width / 2., -self.height / 2., -self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(self.width / 2., self.height / 2., -self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(-self.width / 2., -self.height / 2., self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(-self.width / 2., self.height / 2., self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(self.width / 2., -self.height / 2., self.depth / 2.)
                    .rotate_by(self.rotation),
            self.pos
                + Vector3::new(self.width / 2., self.height / 2., self.depth / 2.)
                    .rotate_by(self.rotation),
        ]
    }

    pub fn sap_vectors(&self) -> [Vector3; 26] {
        let out = [
            Vector3::new(-self.width / 2., -self.height / 2., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(-self.width / 2., self.height / 2., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., -self.height / 2., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., self.height / 2., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(-self.width / 2., -self.height / 2., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(-self.width / 2., self.height / 2., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., -self.height / 2., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., self.height / 2., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            //tmp
            Vector3::new(-self.width / 2., 0., 0.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., 0., 0.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0., -self.height / 2., 0.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0., self.height / 2., 0.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0., 0., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0., 0., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            //tmp2
            Vector3::new(-self.width / 2., -self.height / 2., 0.0)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(-self.width / 2., self.height / 2., 0.0)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., -self.height / 2., 0.0)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., self.height / 2., 0.0)
                .rotate_by(self.rotation)
                .normalized(),
            //tmp3
            Vector3::new(0.0, -self.height / 2., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0.0, -self.height / 2., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0.0, self.height / 2., -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(0.0, self.height / 2., self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            //tmp4
            Vector3::new(-self.width / 2., 0.0, -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(-self.width / 2., 0.0, self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., 0.0, -self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
            Vector3::new(self.width / 2., 0.0, self.depth / 2.)
                .rotate_by(self.rotation)
                .normalized(),
        ];
        out
    }

    pub fn check_collision(&self, other: &Self) -> bool {
        let norms = [self.sap_vectors(), other.sap_vectors()];
        let sverts = self.as_vertices();
        let overts = other.as_vertices();
        for i in norms.iter().flatten() {
            let mut smin = sverts[0].dot(*i);
            let mut smax = smin;
            let mut omin = overts[0].dot(*i);
            let mut omax = omin;
            for j in 1..8 {
                let d = sverts[j].dot(*i);
                if d > smax {
                    smax = d;
                }
                if d < smin {
                    smin = d;
                }
            }
            for j in 1..8 {
                let d = overts[j].dot(*i);
                if d > omax {
                    omax = d;
                }
                if d < smin {
                    omin = d;
                }
            }
            if (smin < omin && smax < omin) || (smax > omax && smin > omax) {
                return false;
            }
        }
        true
    }

    pub fn normals(&self) -> [Vector3; 6] {
        [
            Vector3::new(-1., 0., 0.).rotate_by(self.rotation),
            Vector3::new(1., 0., 0.).rotate_by(self.rotation),
            Vector3::new(0., -1., 0.).rotate_by(self.rotation),
            Vector3::new(0., 1., 0.).rotate_by(self.rotation),
            Vector3::new(0., 0., -1.).rotate_by(self.rotation),
            Vector3::new(0., 0., 1.).rotate_by(self.rotation),
        ]
    }

    pub fn raycast(&self, start: Vector3, direction: Vector3) -> Option<ColData3D> {
        let start = start - self.pos;
        let trans = Quaternion::inverted(&self.rotation);
        let dir = direction.rotate_by(trans);
        let bounds = BoundingBox::new(
            Vector3::new(-self.width / 2.0, -self.height / 2.0, -self.depth / 2.0),
            Vector3::new(self.width / 2.0, self.height / 2.0, self.depth / 2.0),
        );
        let c = bounds.get_ray_collision_box(Ray {
            position: start,
            direction: dir,
        });
        if c.hit {
            let n = c.normal.rotate_by(self.rotation);
            let pos = c.point.rotate_by(self.rotation) + self.pos;
            Some(ColData3D {
                pos,
                normal: n,
                dist: c.distance,
            })
        } else {
            None
        }
    }
}

pub fn line_distance_2d(start1: Vector2, end1: Vector2, start2: Vector2, end2: Vector2) -> f32 {
    let l1 = |f: f32| start1 + end1 * f;
    let l2 = |f: f32| start2 + end2 * f;
    let func = |f: Vector2| l1(f.x).distance_to(l2(f.y));
    newtons_method_2d(func).1
}

pub fn line_distance_3d(start1: Vector3, end1: Vector3, start2: Vector3, end2: Vector3) -> f32 {
    let l1 = |f: f32| start1 + end1 * f;
    let l2 = |f: f32| start2 + end2 * f;
    let func = |f: Vector2| l1(f.x).distance_to(l2(f.y));
    newtons_method_2d(func).1
}

pub fn newtons_method_2d<T: FnMut(Vector2) -> f32>(mut func: T) -> (Vector2, f32) {
    let mut start = Vector2::zero();
    let epsx = Vector2::new(0.01, 0.00);
    let epsy = Vector2::new(0.00, 0.01);
    let mut count = 0;
    loop {
        let base = func(start);
        let dx = ((func(start + epsx) - base) - (func(start - epsx) - base)) / 2.;
        let dy = ((func(start + epsy) - base) - (func(start - epsy) - base)) / 2.;
        let mut hit = false;
        if dx.abs() > 0.001 {
            hit = true;
            start.x += base / dx;
        }
        if dy.abs() > 0.001 {
            hit = true;
            start.y += base / dy;
        }
        if !hit {
            break (start, base);
        }
        count += 1;
        if count > 100 {
            break (start, base);
        }
    }
}

pub fn newtons_method_1d<T: FnMut(f32) -> f32>(mut func: T) -> (f32, f32) {
    let mut start = 0.0;
    let eps = 0.01;
    let mut count = 0;
    loop {
        let base = func(start);
        let d = ((func(start + eps) - base) - (func(start - eps) - base)) / 2.;
        let mut hit = false;
        if d.abs() > 0.001 {
            hit = true;
            start += base / d;
        }
        if !hit {
            break (start, base);
        }
        count += 1;
        if count > 100 {
            break (start, base);
        }
    }
}

pub fn newtons_method_3d<T: FnMut(Vector3) -> f32>(mut func: T) -> (Vector3, f32) {
    let mut start = Vector3::zero();
    let epsx = Vector3::new(0.01, 0.00, 0.0);
    let epsy = Vector3::new(0.00, 0.01, 0.0);
    let epsz = Vector3::new(0.0, 0.0, 0.01);
    let mut count = 0;
    loop {
        let base = func(start);
        let dx = ((func(start + epsx) - base) - (func(start - epsx) - base)) / 2.;
        let dy = ((func(start + epsy) - base) - (func(start - epsy) - base)) / 2.;
        let dz = ((func(start + epsz) - base) - (func(start - epsz) - base)) / 2.;
        let mut hit = false;
        if dx.abs() > 0.001 {
            hit = true;
            start.x += base / dx;
        }
        if dy.abs() > 0.001 {
            hit = true;
            start.y += base / dy;
        }
        if dz.abs() > 0.001 {
            hit = true;
            start.z += base / dz;
        }
        if !hit {
            break (start, base);
        }
        count += 1;
        if count > 100 {
            break (start, base);
        }
    }
}

pub fn newtons_method<const COUNT: usize, T: FnMut(&[f32; COUNT]) -> f32>(
    mut func: T,
) -> ([f32; COUNT], f32) {
    let mut start = [0.0; COUNT];
    let eps = 0.01;
    let mut count = 0;
    loop {
        let base = func(&start);
        let mut deltas = [0.0; COUNT];
        for i in 0..count {
            let mut tmp = start;
            tmp[i] += eps;
            let d1 = func(&tmp) - base;
            tmp = start;
            tmp[i] -= eps;
            let d2 = func(&tmp) - base;
            deltas[i] = (d1 - d2) / 2.0;
        }
        let mut hit = false;
        for i in 0..COUNT {
            if deltas[i].abs() > 0.001 {
                hit = true;
                start[i] += base / deltas[i];
            }
        }
        if !hit {
            break (start, base);
        }
        count += 1;
        if count > 100 {
            break (start, base);
        }
    }
}

pub fn point_distance_to_line_2d(p0: Vector2, p1: Vector2, p2: Vector2) -> f32 {
    let num = ((p2.y - p1.y) * p0.x - (p2.x - p1.x) * p0.y + p2.x * p1.y - p2.y * p1.x).abs();
    let denom = ((p2.y - p1.y) * (p2.y - p1.y) + (p2.x - p1.x) * (p2.x - p1.x)).sqrt();
    num / denom
}
