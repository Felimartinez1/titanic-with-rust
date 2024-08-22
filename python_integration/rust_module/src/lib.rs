use pyo3::prelude::*;
use polars::prelude::*;
use rust_eda::preprocessin;
use rust_eda::algorithm;

#[pyfunction]
fn process_and_run_algorithm(file_path: &str) -> PyResult<String> {
    // Intentar cargar y procesar los datos
    let df = preprocessing::load_and_process_data(file_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error loading data: {:?}", e)))?;
    
    // Intentar ejecutar el algoritmo
    let result_df = algorithm::custom_algorithm(&df)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error running algorithm: {:?}", e)))?;
    
    // Devolver el resultado como una cadena
    Ok(result_df.to_string())
}

#[pymodule]
fn rust_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_and_run_algorithm, m)?)?;
    Ok(())
}
