use super::{Point, Point3D};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) struct Point4D {
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) z: isize,
    pub(crate) w: isize,
}

impl Point for Point4D {
    fn get_vicinity(&self) -> Vec<Self> {
        let mut res = vec![
            Point4D {
                x: self.x,
                y: self.y,
                z: self.z,
                w: self.w - 1,
            },
            Point4D {
                x: self.x,
                y: self.y,
                z: self.z,
                w: self.w + 1,
            },
        ];

        for i in 0..3 {
            let w = if i == 2 { -1 } else { i as isize };

            res.push(Point4D {
                x: self.x,
                y: self.y,
                z: self.z - 1,
                w: self.w + w,
            });

            res.push(Point4D {
                x: self.x,
                y: self.y,
                z: self.z + 1,
                w: self.w + w,
            });

            for j in 0..3 {
                let z = if j == 2 { -1 } else { j as isize };

                res.push(Point4D {
                    x: self.x - 1,
                    y: self.y,
                    z: self.z + z,
                    w: self.w + w,
                });
                res.push(Point4D {
                    x: self.x + 1,
                    y: self.y,
                    z: self.z + z,
                    w: self.w + w,
                });
                res.push(Point4D {
                    x: self.x,
                    y: self.y - 1,
                    z: self.z + z,
                    w: self.w + w,
                });
                res.push(Point4D {
                    x: self.x,
                    y: self.y + 1,
                    z: self.z + z,
                    w: self.w + w,
                });

                res.push(Point4D {
                    x: self.x - 1,
                    y: self.y - 1,
                    z: self.z + z,
                    w: self.w + w,
                });
                res.push(Point4D {
                    x: self.x + 1,
                    y: self.y + 1,
                    z: self.z + z,
                    w: self.w + w,
                });
                res.push(Point4D {
                    x: self.x + 1,
                    y: self.y - 1,
                    z: self.z + z,
                    w: self.w + w,
                });
                res.push(Point4D {
                    x: self.x - 1,
                    y: self.y + 1,
                    z: self.z + z,
                    w: self.w + w,
                });
            }
        }

        res
    }
}

impl From<Point3D> for Point4D {
    fn from(p3d: Point3D) -> Self {
        Point4D {
            x: p3d.x,
            y: p3d.y,
            z: p3d.z,
            w: 0,
        }
    }
}
