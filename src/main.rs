#[macro_use]
extern crate impl_ops;
use std::ops;
use std::{
    cell::RefCell,
    fmt::{self, Debug},
    ops::{Deref, Neg},
    rc::Rc,
};

#[macro_export]
macro_rules! value {
    ( $x:expr ) => {{
        Value::from($x)
    }};
}

#[derive(Debug)]
struct Value(Rc<RefCell<ValueData>>);

impl Deref for Value {
    type Target = Rc<RefCell<ValueData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl_op_ex!(+ |a: &Value, b: &Value| -> Value {
    let out = value!(a.borrow().data + b.borrow().data);
    out.borrow_mut()._prev = vec![Value(Rc::clone(&a)), Value(Rc::clone(&b))];
    out.borrow_mut()._backward = Some(|value: &ValueData| {
        value._prev[0].borrow_mut().grad += value.grad;
        value._prev[1].borrow_mut().grad += value.grad;
    });
    out
});

impl_op_ex!(-|a: &Value, b: &Value| -> Value { a + (-b) });

impl_op_ex!(*|a: &Value, b: &Value| -> Value {
    let out = value!(a.borrow().data * b.borrow().data);
    out.borrow_mut()._prev = vec![Value(Rc::clone(&a)), Value(Rc::clone(&b))];
    out.borrow_mut()._backward = Some(|value: &ValueData| {
        value._prev[0].borrow_mut().grad += value._prev[1].borrow_mut().data * value.grad;
        value._prev[1].borrow_mut().grad += value._prev[0].borrow_mut().data * value.grad;
    });
    out
});

impl_op_ex!(/ |a: &Value, b: &Value| -> Value {
    a * b.pow(&value!(-1.0))
});

type BackwardsFn = fn(value: &ValueData);

struct ValueData {
    data: f64,
    grad: f64,
    _backward: Option<BackwardsFn>,
    _prev: Vec<Value>,
}

impl ValueData {
    fn new(data: f64) -> ValueData {
        ValueData {
            data,
            grad: 0.0,
            _backward: None,
            _prev: Vec::new(),
        }
    }
}

impl Debug for ValueData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value")
            .field("data", &self.data)
            .field("grad", &self.grad)
            .finish()
    }
}

impl<T: Into<f64>> From<T> for Value {
    fn from(t: T) -> Value {
        Value::new(ValueData::new(t.into()))
    }
}

impl Value {
    fn new(value: ValueData) -> Value {
        Value(Rc::new(RefCell::new(value)))
    }

    fn relu(&self) -> Value {
        let out = value!(self.borrow().data.max(0.0));
        out.borrow_mut()._prev = vec![Value(Rc::clone(&self))];
        out.borrow_mut()._backward = Some(|value: &ValueData| {
            value._prev[0].borrow_mut().grad += value.grad;
        });
        out
    }

    fn pow(&self, other: &Value) -> Value {
        let out = value!(self.borrow().data.powf(other.borrow().data));
        out.borrow_mut()._prev = vec![Value(Rc::clone(&self)), Value(Rc::clone(&other))];
        out.borrow_mut()._backward = Some(|value: &ValueData| {
            let base = value._prev[0].borrow().data;
            let p = value._prev[1].borrow().data;
            value._prev[0].borrow_mut().grad += p * base.powf(p - 1.0) * value.grad;
        });
        out
    }
}

impl Neg for &Value {
    type Output = Value;

    fn neg(self) -> Value {
        let neg = value!(-1.0);
        self * &neg
    }
}

fn main() {
    // Micrograd:
    // a = Value(-4.0)
    let a = value!(-4.0);
    // b = Value(2.0)
    let b = value!(2.0);
    // c = a + b
    let mut c = &a + &b;
    // d = a * b + b**3
    let mut d = &a * &b + &b.pow(&value!(3.0));
    // c += c + 1
    c = &c + &c + value!(1.0);
    // c += 1 + c + (-a)
    c = &c + value!(1.0) + &c + (-&a);
    // d += d * 2 + (b + a).relu()
    d = &d + &d * value!(2.0) + (&b + &a).relu();
    // d += 3 * d + (b - a).relu()
    d = &d + value!(3.0) * &d + (&b - &a).relu();
    // e = c - d
    let e = &c - &d;
    // f = e**2
    let f = e.pow(&value!(2.0));
    // g = f / 2.0
    let mut g = &f / &value!(2.0);
    // g += 10.0 / f
    g = &g + &value!(10.0) / &f;

    // print(f'{g.data:.4f}') # prints 24.7041, the outcome of this forward pass
    println!("{:.4}", g.borrow().data); // 24.7041

    // g.backward()
    // print(f'{a.grad:.4f}') # prints 138.8338, i.e. the numerical value of dg/da
    // print(f'{b.grad:.4f}') # prints 645.5773, i.e. the numerical value of dg/db

    // println!("a is {:?}", a);
    // println!("b is {:?}", b);
    // println!("c is {:?}", c);
    // println!("d is {:?}", d);
    // println!("e is {:?}", e);
    // println!("f is {:?}", f);
    // println!("g is {:?}", g);
}
