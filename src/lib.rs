use pyo3::prelude::*;
use pyo3::class::basic::PyObjectProtocol;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Deserialize, Serialize, Clone)]
struct User {
    #[pyo3(get, set)]
    name: String,
    gender: String,
    weight: f32,
    height: f32,
    age: f32,
}


#[pymethods]
impl User {

    #[new]
    fn new(name: String, gender: String, weight: f32, height: f32, age: f32) -> Self {
        Self {
            name: name,
            gender: gender,
            weight: weight,
            height: height,
            age: age,
        }
    }

    fn basal_metabolic_rate(&self) -> f32 {
        match self.gender.as_ref() {
            "male" => 10.0 * self.weight + 6.25 * self.height - 5.0 * self.age + 4.0,
            "female" => 10.0 * self.weight + 6.25 * self.height - 5.0 * self.age - 161.0,
            _ => 0.0,
        }
    }

    
}

#[pyproto]
impl PyObjectProtocol for User {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "User(name={}, gender={}, weight={}, height={}, age={})",
            self.name,
            self.gender,
            self.weight,
            self.height,
            self.age
        ))
    }
}

#[pyfunction]
fn max(users: Vec<User>) -> PyResult<User> {
    let u = users
        .iter()
        .reduce(|a, b| {
            if a.basal_metabolic_rate() >= b.basal_metabolic_rate() {
                a
            } else {
                b
            }
        })
        .unwrap()
        .clone();
    return Ok(u);
}

#[pyfunction]
fn max_par(users: Vec<User>) -> PyResult<User> {
    let u = users
        .into_par_iter()
        .reduce(
            || User::new(String::new(), String::new(), 0_f32, 0_f32, 0_f32),
            |a, b| {
                if a.basal_metabolic_rate() >= b.basal_metabolic_rate() {
                    a
                } else {
                    b
                }
            },
        )
        .clone();
    return Ok(u);
}

#[pymodule]
fn bmr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max, m)?)?;
    m.add_function(wrap_pyfunction!(max_par, m)?)?;
    m.add_class::<User>()?;
    
    Ok(())
}
