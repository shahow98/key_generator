# Key Generator

## using

- ```shell
  >>> key_generator[.exe] --help

  key_generator
  shahow
  Simple program to Key Generator

  USAGE:
      key_generator[.exe] [OPTIONS]

  OPTIONS:
      -h, --help
              Print help information

      -l, --len <LEN>
              Key length

              [default: 10]

      -m, --mode <MODE>
              Generate mode [easy|medium|hard]

              [default: hard]

      -s, --size <SIZE>
              Key size

              [default: 3]

      -V, --version
              Print version information
  ```

- key generate

  ```shell
  >>>key_generator.exe --len 10 --size 3 --mode hard
  UWvc787*(2
  8_4~$+712@
  d0k06#r~~0
  ```

## build

1. required

   [rust 1.60.0](https://www.rust-lang.org/zh-CN/tools/install)

2. building

   ```shell

   cargo build --relase

   ```

3. binary file

   file path: ./target/release/key_generator
