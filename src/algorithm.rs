use polars::prelude::*;
use std::error::Error;

pub fn custom_algorithm(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let mut normalized_df = df.clone();
    let fare_series = df.column("Fare_mean")?;
    
    // Especificar el tipo de max_fare y min_fare
    let max_fare: f64 = fare_series.f64()?.max().unwrap();
    let min_fare: f64 = fare_series.f64()?.min().unwrap();

    let normalized_fare = fare_series
        .f64()?
        .apply(|val| (val - min_fare) / (max_fare - min_fare));

    normalized_df.replace("Fare_mean", normalized_fare.into_series())?;
    Ok(normalized_df)
}
