## Azure functions custom handler in Rust

- Test Rust server with Vs Code

  > Install Rest Client extension by Huachao Mao

  - ```sh
      cd handler && cargo run
    ```
  - Open `'./test_handler.rest'` in Vs code
  - Click on `Send request`

- Test Azure function

  - ```sh
      func start
    ```
  - Open `'./test_function.rest'` in Vs code
  - Click on `Send request`
