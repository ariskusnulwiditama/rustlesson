fn divide(dividend: f64, divisor: f64) ->Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

