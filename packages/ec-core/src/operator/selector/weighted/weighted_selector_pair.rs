use rand::distr::{Bernoulli, Distribution};

use super::error::{WeightSumOverflow, WeightedSelectorsError};
use crate::{operator::selector::Selector, population::Population, with_weight::WithWeight};
