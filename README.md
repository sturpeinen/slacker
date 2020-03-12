# slacker
```
$ slacker -h
slacker 1.1.0

USAGE:
    slacker [FLAGS] [OPTIONS]

FLAGS:
    -h, --help             Prints help information
        --no-rate-limit    Ignore rate limit and send as fast as possible
    -V, --version          Prints version information

OPTIONS:
    -c, --config <config>            Path to config [default: ~/.config/slacker.conf]
    -n, --name <name>                Use named Slack Webhook URL
    -u, --url <url>                  Slack Webhook URL (overrides config)
```

## Usage

Create [Incoming Webhook](https://slack.com/intl/en-fi/help/articles/115005265063) for your Slack workspace and add the Webhook URL to **slacker** config.
```
$ cat ~/.config/slacker.conf
# Default Slack Webhook
slack_hook = "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXR34LD34L"

[hooks]
eggs = "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXX3GG5"
random = "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXR4ND0M"
```

Run **slacker**
```
$ echo "hellolo" | slacker
$ sleep 360 && echo "done" | slacker -n eggs
```

## Build

### Cargo
Install [Rust Toolchain](https://rustup.rs/) and run ```cargo build --release```.
```
$ cd slacker
$ cargo build --release
...
Finished release [optimized] target(s) in 6m 54s

$ du target/release/slacker
6936	target/release/slacker
```

### Docker
You can use [Docker](https://en.wikipedia.org/wiki/Docker_(software)) to build static x86_64 [Linux](https://en.wikipedia.org/wiki/Linux) binary.
```
$ cd slacker
$ ./buildmusl.sh
...

$ du build/slacker 
6688	build/slacker
```

## Bill of Materials

The Bill of Materials ([BOM.txt](./BOM.txt)) created with excellent [cargo-bom](https://github.com/sensorfu/cargo-bom).
```
$ cargo bom
Name       | Version  | Licenses
----       | -------  | --------
dirs       | 2.0.2    | Apache-2.0, MIT
reqwest    | 0.10.4   | Apache-2.0, MIT
serde      | 1.0.104  | Apache-2.0, MIT
structopt  | 0.3.11   | Apache-2.0, MIT
toml       | 0.5.6    | Apache-2.0, MIT
url        | 2.1.1    | Apache-2.0, MIT
...
```
