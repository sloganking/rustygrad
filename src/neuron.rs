use crate::Value;
use rand::{distributions::Uniform, Rng};
use std::fmt::{self, Debug};

pub struct Neuron {
    w: Vec<Value>,
    b: Value,
    nonlin: bool,
}

impl Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(if self.nonlin { "ReLU" } else { "Linear" })
            .field("weights", &self.w)
            .field("bias", &self.b)
            .finish()
    }
}

impl Neuron {
    pub fn new(nin: i32, nonlin: bool) -> Neuron {
        let mut rng = rand::thread_rng();
        let range = Uniform::<f64>::new(-1.0, 1.0);

        Neuron {
            w: (0..nin).map(|_| Value::from(rng.sample(&range))).collect(),
            b: Value::from(0.0),
            nonlin,
        }
    }

    pub fn from(nin: i32) -> Neuron {
        Neuron::new(nin, true)
    }

    pub fn forward(&self, x: &Vec<Value>) -> Value {
        let wixi_sum: f64 = self
            .w
            .iter()
            .zip(x)
            .map(|(wi, xi)| wi.borrow().data * xi.borrow().data)
            .sum();
        let out = Value::from(wixi_sum + self.b.borrow().data);

        if self.nonlin {
            return out.relu();
        }
        out
    }
}
