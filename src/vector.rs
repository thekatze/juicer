use std::mem::MaybeUninit;

#[derive(Debug, Clone)]
pub struct Vector<const SIZE: usize, T>(pub [T; SIZE]);

macro_rules! impl_elementwise_arithmetic_op {
    ($fn:ident, $($bound:tt)*) => {
        impl<const SIZE: usize, T> $($bound)* for Vector<SIZE, T>
        where
            T: $($bound)*<Output = T> + Copy,
        {
            type Output = Self;

            fn $fn(self, rhs: Self) -> Self::Output {
                let mut result = MaybeUninit::uninit_array::<SIZE>();

                for i in 0..SIZE {
                    result[i] = MaybeUninit::new($($bound)*::$fn(self.0[i], rhs.0[i]));
                }

                // SAFETY: we iterated over every index of the array
                Vector(unsafe { MaybeUninit::array_assume_init(result) })
            }
        }
    };
}

impl_elementwise_arithmetic_op!(add, std::ops::Add);
impl_elementwise_arithmetic_op!(sub, std::ops::Sub);

macro_rules! impl_scalar_arithmetic_op {
    ($fn:ident, $($bound:tt)*) => {
        impl<const SIZE: usize, T> $($bound)*<T> for Vector<SIZE, T>
        where
            T: $($bound)*<Output = T> + Copy,
        {
            type Output = Self;

            fn $fn(self, scalar: T) -> Self::Output {
                let mut result = MaybeUninit::uninit_array::<SIZE>();

                for i in 0..SIZE {
                    result[i] = MaybeUninit::new($($bound)*::$fn(self.0[i], scalar));
                }

                // SAFETY: we iterated over every index of the array
                Vector(unsafe { MaybeUninit::array_assume_init(result) })
            }
        }
    };
}

impl_scalar_arithmetic_op!(mul, std::ops::Mul);
impl_scalar_arithmetic_op!(div, std::ops::Div);

impl<const SIZE: usize, T> std::ops::Neg for Vector<SIZE, T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = MaybeUninit::uninit_array::<SIZE>();

        for i in 0..SIZE {
            result[i] = MaybeUninit::new(std::ops::Neg::neg(self.0[i]));
        }

        // SAFETY: we iterated over every index of the array
        Vector(unsafe { MaybeUninit::array_assume_init(result) })
    }
}
impl<const SIZE: usize, T, MulOutput> Vector<SIZE, T>
where
    T: Copy + std::ops::Add + std::ops::Mul<Output = MulOutput>,
    MulOutput: Into<f32>,
{
    pub fn dot(&self, rhs: &Vector<SIZE, T>) -> f32 {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .fold(0.0, |acc, (x, y)| acc + Into::into(*x * *y))
    }
}

impl<const SIZE: usize, T> Vector<SIZE, T>
where
    T: Copy + Into<f32>,
{
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.0
            .iter()
            .fold(0.0, |acc, x| acc + f32::powi((*x).into(), 2))
    }

    pub fn normalize(&self) -> Vector<SIZE, f32> {
        let len = self.len();
        let mut result = MaybeUninit::uninit_array::<SIZE>();
        for i in 0..SIZE {
            result[i] = MaybeUninit::new(self.0[i].into() / len);
        }
        // SAFETY: we iterated over every index of the array
        Vector(unsafe { MaybeUninit::array_assume_init(result) })
    }
}

// convenience
impl<T> Vector<2, T>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }
    pub fn y(&self) -> T {
        self.0[1]
    }
}

impl<T> Vector<3, T>
where
    T: Copy,
{
    pub fn x(&self) -> T {
        self.0[0]
    }
    pub fn y(&self) -> T {
        self.0[1]
    }
    pub fn z(&self) -> T {
        self.0[2]
    }
}

impl<const SIZE: usize, T> core::iter::Sum for Vector<SIZE, T>
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector([T::default(); SIZE]), |acc, x| acc + x)
    }
}
