use polars::prelude::*;
use polars_lazy::prelude::*;  // Importa los módulos necesarios para LazyFrame
use std::fs::File;
use std::error::Error;

pub fn load_and_process_data(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let df = CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Convertir a LazyFrame
    let lazy_df = df.lazy();

    // Filtrar filas donde la columna "Age" sea mayor o igual a 30
    let filtered_df = lazy_df
        .filter(col("Age").gt_eq(30))
        .collect()?; // Usar collect() para obtener el DataFrame filtrado

    // Agrupar por "Pclass" y calcular la media de "Fare"
    let aggregated_df = filtered_df
        .lazy()
        .groupby(["Pclass"])
        .agg([col("Fare").mean().alias("Fare_mean")])
        .collect()?; // Usar collect() para obtener el DataFrame agregado

    Ok(aggregated_df)
}
