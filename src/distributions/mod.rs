//! Probability distributions.

pub use self::bernoulli::Bernoulli;
pub use self::beta::Beta;
pub use self::binomial::Binomial;
pub use self::categorical::Categorical;
pub use self::exponential::Exponential;
pub use self::gamma::Gamma;
pub use self::gaussian::Gaussian;
pub use self::uniform::Uniform;

mod bernoulli;
mod beta;
mod binomial;
mod categorical;
mod exponential;
mod gamma;
mod gaussian;
mod uniform;
