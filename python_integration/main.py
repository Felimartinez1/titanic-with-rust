import rust_module

if __name__ == "__main__":
    file_path = "python_integration/data/titanic2.csv"
    result = rust_module.process_and_run_algorithm(file_path)
    print(f"Result: {result}")