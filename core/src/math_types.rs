pub type Vec2f = vek::Vec2<f32>;
pub type Vec3f = vek::Vec3<f32>;
pub type Vec4f = vek::Vec4<f32>;

pub type Vec2d = vek::Vec2<f64>;
pub type Vec3d = vek::Vec3<f64>;
pub type Vec4d = vek::Vec4<f64>;

pub type Vec2i = vek::Vec2<i32>;
pub type Vec3i = vek::Vec3<i32>;
pub type Vec4i = vek::Vec4<i32>;

pub type Mat2f = vek::mat::column_major::Mat2<f32>;
pub type Mat3f = vek::mat::column_major::Mat3<f32>;
pub type Mat4f = vek::mat::column_major::Mat4<f32>;

pub type Mat2d = vek::mat::column_major::Mat2<f64>;
pub type Mat3d = vek::mat::column_major::Mat3<f64>;
pub type Mat4d = vek::mat::column_major::Mat4<f64>;

pub fn vec2<T>(x: T, y: T) -> vek::Vec2<T> {
    vek::Vec2::new(x, y)
}

pub fn vec3<T>(x: T, y: T, z: T) -> vek::Vec3<T> {
    vek::Vec3::new(x, y, z)
}

pub fn vek4<T>(x: T, y: T, z: T, w: T) -> vek::Vec4<T> {
    vek::Vec4::new(x, y, z, w)
}
