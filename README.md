# Jaundice Lynx
A program built to automate a manual simulation I had to do for my Enviromental Science class.

## Usage
run `cargo run` to build and run the command.\
It will display the output results and create a file named `output.csv` that also contains the data.\
There is a number of flags you can use when using the command:
```
Usage: jaundice_lynx [OPTIONS]                                                                                                                                                                                                              
                                                                                                                                                                                                                                            
Options:                                                                                                                                                                                                                                    
      --start-hares <START_HARES>        [default: 0]
      --start-lynx <START_LYNX>          [default: 0]
      --min-hares <MIN_HARES>            [default: 3]
      --min-lynx <MIN_LYNX>              [default: 1]
      --max-hares <MAX_HARES>            [default: 120]
      --max-lynx <MAX_LYNX>              [default: 8]
      --hares-per-lynx <HARES_PER_LYNX>  [default: 3]
      --hares-size <HARES_SIZE>          [default: 1]
      --lynx-size <LYNX_SIZE>            [default: 3]
  -s, --habitat-size <HABITAT_SIZE>      [default: 12]
  -i, --iterations <ITERATIONS>          [default: 25]
  -o, --output-file <OUTPUT_FILE>        [default: output.csv]
  -h, --help                             Print help
  -V, --version                          Print version
```

## Why is called "Jaundice Lynx?"
it's an inside joke; don't worry about it.
