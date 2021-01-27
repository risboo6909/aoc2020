use super::Point;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) struct Point3D {
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) z: isize,
}

impl Point for Point3D {
    fn get_vicinity(&self) -> Vec<Self> {
        let mut res = vec![
            Point3D {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Point3D {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ];
        for j in 0..3 {
            let z = if j == 2 { -1 } else { j as isize };

            res.push(Point3D {
                x: self.x - 1,
                y: self.y,
                z: self.z + z,
            });
            res.push(Point3D {
                x: self.x + 1,
                y: self.y,
                z: self.z + z,
            });
            res.push(Point3D {
                x: self.x,
                y: self.y - 1,
                z: self.z + z,
            });
            res.push(Point3D {
                x: self.x,
                y: self.y + 1,
                z: self.z + z,
            });

            res.push(Point3D {
                x: self.x - 1,
                y: self.y - 1,
                z: self.z + z,
            });
            res.push(Point3D {
                x: self.x + 1,
                y: self.y + 1,
                z: self.z + z,
            });
            res.push(Point3D {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z + z,
            });
            res.push(Point3D {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z + z,
            });
        }

        res
    }
}
