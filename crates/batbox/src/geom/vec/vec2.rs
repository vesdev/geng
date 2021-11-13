use super::*;

/// 2 dimensional vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vec2<T> {
    /// `x` coordinate of the vector
    pub x: T,
    /// `y` coordinate of the vector
    pub y: T,
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}

/// Construct a 2-d vector with given components.
///
/// # Example
/// ```
/// use batbox::*;
/// let v = vec2(1, 2);
/// ```
pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from(v: [T; 2]) -> Vec2<T> {
        let [x, y] = v;
        vec2(x, y)
    }
}

impl<T> Deref for Vec2<T> {
    type Target = [T; 2];
    fn deref(&self) -> &[T; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for Vec2<T> {
    fn deref_mut(&mut self) -> &mut [T; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> Vec2<T> {
    /// Extend into a 3-d vector.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// assert_eq!(vec2(1, 2).extend(3), vec3(1, 2, 3));
    /// ```
    pub fn extend(self, z: T) -> Vec3<T> {
        vec3(self.x, self.y, z)
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2<U> {
        vec2(f(self.x), f(self.y))
    }
}

impl<T: UNum> Vec2<T> {
    /// A zero 2-d vector
    pub const ZERO: Self = vec2(T::ZERO, T::ZERO);
}

impl<T: Num + Copy> Vec2<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// assert_eq!(Vec2::dot(vec2(1, 2), vec2(3, 4)), 11);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y
    }

    /// Calculate skew product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// assert_eq!(Vec2::skew(vec2(1, 2), vec2(3, 4)), -2);
    /// ```
    pub fn skew(a: Self, b: Self) -> T {
        a.x * b.y - a.y * b.x
    }
}

impl<T: Neg<Output = T>> Vec2<T> {
    /// Rotate a vector by 90 degrees counter clockwise.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(3.0, 4.0);
    /// assert_eq!(v.rotate_90(), vec2(-4.0, 3.0));
    /// ```
    pub fn rotate_90(self) -> Self {
        vec2(-self.y, self.x)
    }
}

impl<T: Float> Vec2<T> {
    /// Normalize a vector.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v: Vec2<f64> = vec2(1.0, 2.0);
    /// assert!((v.normalize().len() - 1.0).abs() < 1e-5);
    /// ```
    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Normalizes a vector unless its length its approximately 0.
    /// Can be used to avoid division by 0.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(1.0, 2.0);
    /// assert_eq!(v.normalize_or_zero(), v.normalize());
    /// let v = vec2(1e-10, 1e-10);
    /// assert_eq!(v.normalize_or_zero(), Vec2::ZERO);
    /// ```
    pub fn normalize_or_zero(self) -> Self {
        let len = self.len();
        if len.approx_eq(&T::ZERO) {
            Vec2::ZERO
        } else {
            self / len
        }
    }

    /// Calculate length of a vector.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(3.0, 4.0);
    /// assert_eq!(v.len(), 5.0);
    /// ```
    pub fn len(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y)
    }

    /// Rotate a vector by a given angle.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(1.0, 2.0);
    /// assert!((v.rotate(std::f32::consts::FRAC_PI_2) - vec2(-2.0, 1.0)).len() < 1e-5);
    /// ```
    pub fn rotate(self, angle: T) -> Self {
        let (sin, cos) = T::sin_cos(angle);
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Clamp vector's length from above.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(1.0, 2.0);
    /// assert_eq!(v.clamp(1.0), v.normalize());
    /// ```
    pub fn clamp(self, max_len: T) -> Self {
        let len = self.len();
        if len > max_len {
            self * max_len / len
        } else {
            self
        }
    }

    /// Clamp vector by `min` and `max` values.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(0.5, 2.0);
    /// let min = vec2(0.0, 0.0);
    /// let max = vec2(1.0, 1.0);
    /// assert_eq!(v.clamp_min_max(min, max), vec2(0.5, 1.0));
    /// ```
    pub fn clamp_min_max(self, min: Self, max: Self) -> Self {
        fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
            if value < min {
                min
            } else if value > max {
                max
            } else {
                value
            }
        }

        vec2(clamp(self.x, min.x, max.x), clamp(self.y, min.y, max.y))
    }

    /// Get an angle between the positive direction of the x-axis.
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let v = vec2(0.0, 1.0);
    /// assert_eq!(v.arg(), std::f32::consts::FRAC_PI_2);
    /// ```
    pub fn arg(self) -> T {
        T::atan2(self.y, self.x)
    }
}
