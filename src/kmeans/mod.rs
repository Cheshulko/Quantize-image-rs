use std::ops::{Add, AddAssign, DivAssign};

use ordered_float::OrderedFloat;
use rand::random;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
    pub z: OrderedFloat<f32>,
}

impl Point {
    fn euclidean_distance_sqr(&self, point: &Point) -> OrderedFloat<f32> {
        let dx = self.x - point.x;
        let dy = self.y - point.y;
        let dz = self.z - point.z;

        let t = dx * dx + dy * dy + dz * dz;

        t
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl DivAssign<usize> for Point {
    fn div_assign(&mut self, rhs: usize) {
        self.x = self.x / rhs as f32;
        self.y = self.y / rhs as f32;
        self.z = self.z / rhs as f32;
    }
}

pub struct KMeans {
    pub clusters: Vec<Vec<Point>>,
    pub means: Vec<Point>,
    pub points: Vec<Point>,

    k: usize,
    min: OrderedFloat<f32>,
    max: OrderedFloat<f32>,
}

impl KMeans {
    pub fn generate_k_means(points: Vec<Point>, k: usize) -> KMeans {
        let mut kmeans = KMeans::new(points, k);
        // TODO: metrix?
        for _ in 0..100 {
            kmeans.update_means();
            kmeans.recluster();
        }

        kmeans
    }

    fn new(points: Vec<Point>, k: usize) -> Self {
        let min = points
            .iter()
            .flat_map(|p| vec![p.x, p.y, p.z])
            .min()
            .expect("Points list is empty");
        let max = points
            .iter()
            .flat_map(|p| vec![p.x, p.y, p.z])
            .max()
            .expect("Points list is empty");

        KMeans {
            clusters: vec![vec![]; k],
            means: vec![Point::default(); k],
            k,
            points,
            min,
            max,
        }
    }

    pub fn cluster_point_for_point(&self, point: &Point) -> Point {
        let (mean, _) = self.cluster_point(point);
        mean.clone()
    }

    fn cluster_point<'a>(&'a self, point: &Point) -> (&'a Point, usize) {
        let (_, (mean, mean_ind)) = self
            .means
            .iter()
            .enumerate()
            .map(|(mean_ind, mean)| (mean.euclidean_distance_sqr(point), (mean, mean_ind)))
            .min_by(|a, b| a.0.cmp(&b.0))
            .expect("Means list is empty");

        (mean, mean_ind)
    }

    fn generate_mean(&self) -> Point {
        Point {
            x: self.min + (self.max - self.min) * random::<f32>(),
            y: self.min + (self.max - self.min) * random::<f32>(),
            z: self.min + (self.max - self.min) * random::<f32>(),
        }
    }

    pub fn update_means(&mut self) {
        for k in 0..self.k {
            if self.clusters[k].is_empty() {
                self.means[k] = self.generate_mean();
            } else {
                self.means[k] = Point::default();
                for point_at_cluster in self.clusters[k].iter() {
                    self.means[k] += *point_at_cluster;
                }
                self.means[k] /= self.clusters[k].len();
            }
        }
    }

    pub fn recluster(&mut self) {
        for cluster in self.clusters.iter_mut() {
            cluster.clear();
        }

        for point in self.points.iter() {
            let (_, mean_ind) = self.cluster_point(point);
            self.clusters[mean_ind].push(point.clone());
        }
    }
}
