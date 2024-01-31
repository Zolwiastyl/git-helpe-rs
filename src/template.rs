use anyhow::{Error, Result};

pub fn validate_interpolation_places_count(format: &String, params_len: usize) -> Result<()> {
    let splitted = format.split("{}").into_iter();
    let places_to_interpolate = Vec::from_iter(splitted).len();

    if places_to_interpolate - 1 == params_len {
        Ok(())
    } else {
        Err(Error::msg(format!(
            "
       \n Number of places to interpolate doesn't match with number of args provided.
       Expected {} 
       Received {}
        ",
            places_to_interpolate - 1,
            params_len
        )))
    }
}

pub fn interpolate(format: &String, values: Vec<String>) -> Result<String> {
    let splitted = format.split("{}");
    let appended: Vec<String> = splitted
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            if i >= values.len() {
                return x.to_owned();
            }
            return x.to_owned() + &values[i];
        })
        .collect();

    Ok(appended.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_no_placeholders() {
        let format = String::from("Hello, world!");
        let values = vec![];
        let result = interpolate(&format, values);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[test]
    fn test_interpolate_single_placeholder() {
        let format = String::from("Hello, {}!");
        let values = vec![String::from("world")];
        let result = interpolate(&format, values);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[test]
    fn test_interpolate_multiple_placeholders() {
        let format = String::from("Hello, {}, you are {} years old.");
        let values = vec![String::from("John"), String::from("30")];
        let result = interpolate(&format, values);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, John, you are 30 years old.");
    }

    #[test]
    fn test_interpolate_not_enough_values() {
        let format = String::from("Hello, {}!");
        let values: Vec<String> = vec![];
        let result = validate_interpolation_places_count(&format, values.len());
        assert!(result.is_err());
    }

    #[test]
    fn test_interpolate_too_many_values() {
        let format = String::from("Hello, {}!");
        let values = vec![String::from("world"), String::from("extra")];
        let result = validate_interpolation_places_count(&format, values.len());
        assert!(result.is_err());
    }
}
