use std::ops::{Mul, Div, Deref, DerefMut, Add, Sub};

use num_traits::Float;

pub mod quality;


#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Gaussian<T> where T: Float {
    pub pi: T,
    pub tau: T,
}

impl<T> Default for Gaussian<T>
where T: Float
{
    fn default() -> Self {
        Self { pi: T::zero(), tau: T::zero() }
    }
}

impl<T> Gaussian<T> where T: Float{
    pub fn new(mu: T, sigma: T) -> Self {
        let pi = sigma.powi(2);
        let tau = pi * mu;
        Self { pi, tau }
    }

    pub fn mu(&self) -> T {
        if self.pi == T::zero() { T::zero() } else { self.tau / self.pi }
    }

    pub fn sigma(&self) -> T {
        return if self.pi == T::zero() { T::zero() } else { T::sqrt(T::from(1).unwrap() / self.pi)} 
    }

    pub fn from_mu_sigma(mu: T, sigma: T) -> Self {
        let pi = T::from(1).unwrap()/sigma.powi(2);
        return Self { pi, tau: pi * mu }
    }
}

impl<T> Mul for Gaussian<T> where T: Float {
    type Output = Gaussian<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { pi: self.pi+rhs.pi, tau: self.tau+rhs.tau }
    }
}

impl<T> Div for Gaussian<T> where T: Float {
    type Output = Gaussian<T>;
    fn div(self, rhs: Self) -> Self::Output {
        Self { pi: self.pi-rhs.pi, tau: self.tau-rhs.tau }
    }
}



#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Rating<T> where T: Float {
    gaussian: Gaussian<T>,
}

impl<T> Default for Rating<T>
where T: Float
{
    fn default() -> Self {
        Self { gaussian: Default::default() }
    }
}

impl<T> Rating<T> where T: Float
{
    pub fn new(mu: T, sigma: T) -> Self { Self { gaussian: Gaussian::from_mu_sigma(mu, sigma) } }
   
 
    pub fn mu(&self) -> T {
        return self.gaussian.mu()
    }

    pub fn sigma(&self) -> T {
        return self.gaussian.sigma()
    }
}

impl<T> From<Gaussian<T>> for Rating<T> where T: Float {
    fn from(gaussian: Gaussian<T>) -> Self {
        Self { gaussian }
    }
}

impl<T> Add for Rating<T> where T: Float {
    type Output = Rating<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.gaussian * rhs.gaussian)
    }
}

impl<T> Sub for Rating<T> where T: Float {
    type Output = Rating<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.gaussian / rhs.gaussian)
    }
}

impl<T> Deref for Rating<T> where T: Float{
    type Target = Gaussian<T>;
    fn deref(&self) -> &Self::Target {
        &self.gaussian
    }
}

impl<T> DerefMut for Rating<T> where T: Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gaussian
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

