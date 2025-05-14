pub fn add(numbers: Vec<f64>) -> f64 {
    numbers.iter().fold(0.0, |acc, num| acc + num)
}

pub fn subtract<T, U>(num1: T, num2: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    num1.into() - num2.into()
}
