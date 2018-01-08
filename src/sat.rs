use utils::{min, max};
use vecmath::*;

use std::f32;

pub enum ShapeType {
	Convex,
	Circle,
}

// min, max
pub type Interval = Vector2<f32>;

pub trait Shape {
	fn get_type(&self) -> ShapeType;
	fn set_position(&mut self, position: Vector2<f32>);
	fn get_position(&self) -> Vector2<f32>;
	fn get_verts(&self) -> Vec<Vector2<f32>>;
	fn get_normals_to_test_against(&self, other_shape: &Shape) -> Vec<Vector2<f32>>;
	fn calculate_projection_on(&self, vector: &Vector2<f32>) -> Interval;
}

#[derive(Debug)]
pub struct ConvexObject {
	pub position: Vector2<f32>,

	// Note: clock wise winding, origin at 0.0
	pub verts: Vec<Vector2<f32>>,
	pub scale: Vector2<f32>, 
}

impl ConvexObject {
	pub fn new(position: Vector2<f32>, verts: Vec<Vector2<f32>>) -> Self {
		ConvexObject {
			position: position,
			verts: verts,
			scale: [1.0, 1.0], //For now
		}
	}

	fn calculate_normals(&self) -> Vec<Vector2<f32>> {
		assert!(self.verts.len() >= 3);
		let mut result = Vec::new();
		let mut prev = &self.verts[0];
		let vert_counts = self.verts.len(); 
		for i in 1..(vert_counts + 1) {
			let index = i % vert_counts;
			let cur = &self.verts[index];
			result.push(calculate_normal(*prev, *cur));
			prev = cur;
		}
		result
	}

	// Note: only position offset and scale supported for now 
	fn transfrom_vec(&self, vector: &Vector2<f32>) -> Vector2<f32> {
		vec2_mul(vec2_add(*vector, self.get_position()), self.scale)
	}
}

impl Shape for ConvexObject {
	fn get_type(&self) -> ShapeType {
		ShapeType::Convex
	}

	fn set_position(&mut self, position: Vector2<f32>) {
		self.position = position;
	}

	fn get_position(&self) -> Vector2<f32> {
		self.position
	}

	fn get_verts(&self) -> Vec<Vector2<f32>> {
		self.verts.clone()
	}
	
	fn get_normals_to_test_against(&self, _other_shape: &Shape) -> Vec<Vector2<f32>> {
		self.calculate_normals()
	}
	
	fn calculate_projection_on(&self, normal: &Vector2<f32>) -> Interval {
		let mut cur_min = f32::MAX;
		let mut cur_max = f32::MIN;
		for vert in &self.verts {
			let cur = self.transfrom_vec(vert);
			let dot = vec2_dot(*normal, cur);
			if dot < cur_min {
				cur_min = dot;
			}
			if dot > cur_max {
				cur_max = dot;
			}
		}
		[cur_min, cur_max]
	}
}

#[derive(Debug)]
pub struct CircleObject {
	pub position: Vector2<f32>,
	pub radius: f32,
	pub scale: Vector2<f32>, 
}

impl CircleObject {
	pub fn new(position: Vector2<f32>, radius: f32) -> Self {
		CircleObject {
			position: position,
			radius: radius,
			scale: [1.0, 1.0], //For now
		}
	}
}

impl Shape for CircleObject {
	fn get_type(&self) -> ShapeType {
		ShapeType::Circle
	}

	fn set_position(&mut self, position: Vector2<f32>) {
		self.position = position;
	}

	fn get_position(&self) -> Vector2<f32> {
		self.position
	}

	fn get_verts(&self) -> Vec<Vector2<f32>> {
		unimplemented!()
	}
	
	fn get_normals_to_test_against(&self, other_shape: &Shape) -> Vec<Vector2<f32>> {
		let mut result = Vec::new();
		match other_shape.get_type() {
			ShapeType::Circle => {
				result.push(vec2_normalized(vec2_sub(other_shape.get_position(), self.position)));
			}
			ShapeType::Convex => {
				for vert in other_shape.get_verts() {
					result.push(vec2_normalized(vec2_sub(vert, self.position)));
				}
			}
		}
		result
	}
	
	fn calculate_projection_on(&self, normal: &Vector2<f32>) -> Interval {
		let dot = vec2_dot(*normal, self.get_position());
		[dot - self.radius, dot + self.radius]
	}
}

// minimum displacement and normal
pub type CollistionInfo = Option<(Vector2<f32>, Vector2<f32>)>;

pub fn get_collision(a: &Shape, b: &Shape) -> CollistionInfo {
	let mut min_overlap_len = f32::MAX;
	let mut min_overlap_vec = [0.0, 0.0];
	for normals_to_test in [a.get_normals_to_test_against(b), b.get_normals_to_test_against(a)].iter() {
		for normal in normals_to_test {
			let proj_a = a.calculate_projection_on(&normal);
			let proj_b = b.calculate_projection_on(&normal);
			let overlap = calculate_overlap(&proj_a, &proj_b);
			match overlap {
				Some(overlap) => {
					let lenght = overlap[1] - overlap[0];
					if lenght < min_overlap_len {
						min_overlap_len = lenght;
						min_overlap_vec = vec2_sub(
							vec2_mul(*normal, [overlap[1], overlap[1]]), 
							vec2_mul(*normal, [overlap[0], overlap[0]]));
					}
				},
				None => {
					return None
				}
			}
		}
	}
	let direction = vec2_sub(b.get_position(), a.get_position());
	let direction = vec2_dot(direction, min_overlap_vec);
	if direction < 0.0 {
		Some((min_overlap_vec, vec2_normalized(min_overlap_vec)))
	} else {
		Some((vec2_neg(min_overlap_vec), vec2_normalized(vec2_neg(min_overlap_vec))))
	}
}

fn calculate_normal(a: Vector2<f32>, b: Vector2<f32>) -> Vector2<f32> {
	let diff = vec2_sub(b, a);
	vec2_normalized([-diff[1], diff[0]])
}

fn calculate_overlap(a: &Interval, b: &Interval) -> Option<Interval> {
	if a[0] < b[1] && a[1] > b[0] {
		return Some([max(a[0], b[0]), min(a[1], b[1])])
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_overlap() {
		eq([1.0, 2.0], calculate_overlap(&[0.0, 2.0], &[1.0, 5.0]).unwrap());
		eq([1.0, 2.0], calculate_overlap(&[1.0, 5.0], &[0.0, 2.0]).unwrap());
		assert_eq!(None, calculate_overlap(&[1.0, 2.0], &[3.0, 4.0]));
		assert_eq!(None, calculate_overlap(&[3.0, 4.0], &[1.0, 2.0]));
	}

	#[test]
	fn test_collision_1() {
		let verts = vec![[-1.0, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, -1.0]];
		let obj_a = ConvexObject::new([0.0, 0.0], verts.clone());
		let obj_b = ConvexObject::new([1.0, 0.0], verts.clone());
		let collision = get_collision(&obj_a, &obj_b);
		eq(collision.unwrap().0, [-1.0, 0.0]);

		let collision = get_collision(&obj_b, &obj_a);
		eq(collision.unwrap().0, [1.0, 0.0]);
	}

	#[test]
	fn test_collision_2() {
		let verts = vec![[-1.0, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, -1.0]];
		let obj_a = ConvexObject::new([0.0, 0.0], verts.clone());
		let obj_b = ConvexObject::new([3.0, 0.0], verts.clone());
		let collision = get_collision(&obj_a, &obj_b);
		assert_eq!(collision, None);

		let collision = get_collision(&obj_b, &obj_a);
		assert_eq!(collision, None);
	}

	#[test]
	fn test_collision_3() {
		let obj_a = CircleObject::new([0.0, 0.0], 3.0);
		let obj_b = CircleObject::new([3.0, 3.0], 2.0);
		let collision = get_collision(&obj_a, &obj_b);
		eq(collision.unwrap().0, [-0.5355338, -0.5355338]);

		let collision = get_collision(&obj_b, &obj_a);
		eq(collision.unwrap().0, [0.5355338, 0.5355338]);
	}

	#[test]
	fn test_collision_4() {
		let obj_a = CircleObject::new([0.0, 0.0], 3.0);
		let obj_b = CircleObject::new([4.0, 4.0], 2.0);
		let collision = get_collision(&obj_a, &obj_b);
		assert_eq!(collision, None);

		let collision = get_collision(&obj_b, &obj_a);
		assert_eq!(collision, None);
	}

	#[test]
	fn test_collision_5() {
		let verts = vec![[-1.0, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, -1.0]];
		let obj_a = ConvexObject::new([0.0, 0.0], verts.clone());
		let obj_b = CircleObject::new([2.0, 0.0], 1.5);
		let collision = get_collision(&obj_a, &obj_b);
		eq(collision.unwrap().0, [-0.5, 0.0]);

		let collision = get_collision(&obj_b, &obj_a);
		eq(collision.unwrap().0, [0.5, 0.0]);
	}

	#[test]
	fn test_collision_6() {
		let verts = vec![[-1.0, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, -1.0]];
		let obj_a = ConvexObject::new([0.0, 0.0], verts.clone());
		let obj_b = CircleObject::new([2.0, 2.0], 2.0);
		let collision = get_collision(&obj_a, &obj_b);
		eq(collision.unwrap().0, [-0.41421348, -0.41421348]);

		let collision = get_collision(&obj_b, &obj_a);
		eq(collision.unwrap().0, [0.41421348, 0.41421348]);
	}

	fn eq(a: Vector2<f32>, b: Vector2<f32>) {
		assert!((a[0] - b[0]).abs() < 0.000001);
		assert!((a[1] - b[1]).abs() < 0.000001);
	}
}
