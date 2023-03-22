# posh

# Description
This is a command-line tool which obtains the number of listings for the given brand (and optional query term) from the website Poshmark.

## Usage

```
Usage: posh [OPTIONS] <BRAND> [QUERY]

Arguments:
  <BRAND>  The brand to be searched for. Use quotation marks if there is a space in the name. e.g "Old Navy"
  [QUERY]  An optional search term to include with the brand. e.g. sandals or "tweed suit"

Options:
  -s, --step <STEP>  The number of results to fetch for each page (limited by Poshmark to 100) [default: 100]
```

### Basic usage example
To get the number of Gucci items listed on Poshmark:

```
RUST_LOG=info cargo run Gucci
```


### Specifying step size

If you want to set a different number of results per page than the default 100, you can use the --step argument. Note that you'll need to include -- after cargo run preceding the arguments if using this parameter.

```
RUST_LOG=info cargo run -- -step 500 nike airmax
```


### Logging

If logging is desired, run with the desired value of the RUST\_LOG environment variable (info,debug,trace,warn, or error:

```
RUST_LOG=debug cargo run "Oscar de la Renta" sweater
```
