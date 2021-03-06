use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};

use reqwest::blocking::Client;
use serde::Deserialize;
use structopt::StructOpt;
use url::Url;

// Interval (in ms) between posts
// https://api.slack.com/docs/rate-limits
const POST_INTERVAL: u64 = 1000;

#[derive(Debug, StructOpt)]
struct Opts {
    /// Path to config
    #[structopt(short, long, default_value = "~/.config/slacker.conf")]
    config: PathBuf,

    /// Slack Webhook URL (overrides config)
    #[structopt(short, long)]
    url: Option<Url>,

    /// Use named Slack Webhook URL
    #[structopt(short, long)]
    name: Option<String>,

    /// Ignore rate limit and send as fast as possible
    #[structopt(long)]
    no_rate_limit: bool,
}

#[derive(Deserialize)]
struct Config {
    slack_hook: Option<Url>,
    hooks: Option<HashMap<String, Url>>,
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let opts = Opts::from_args();

    let url = if let Some(url) = opts.url {
        url
    } else {
        match read_config(opts.config) {
            Ok(Config {
                hooks: Some(hooks), ..
            }) if opts.name.is_some() => {
                let name: String = opts.name.unwrap();
                if let Some(hook) = hooks.get(&name) {
                    hook.clone()
                } else {
                    eprintln!("Could not find Slack Webhook '{}'", name);
                    return 1;
                }
            }
            Ok(Config {
                slack_hook: Some(hook),
                ..
            }) => hook,
            Ok(Config {
                slack_hook: None, ..
            }) => {
                eprintln!("Missing default Slack Webhook");
                return 1;
            }
            Err(err) => {
                eprintln!("Failed to read config ({})", err);
                return 1;
            }
        }
    };

    let mut l = Limiter::default();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                if !opts.no_rate_limit {
                    l.limit();
                }
                if let Err(err) = post(&url, &input.trim_end()) {
                    eprintln!("Failed to send message ({})", err);
                }
            }
            Err(err) => {
                eprintln!("Failed to read input ({}).", err);
                return 1;
            }
        }
    }
    0
}

fn read_config(path: PathBuf) -> io::Result<Config> {
    let config_path = if let Ok(p) = path.strip_prefix("~/") {
        let mut base = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

        base.push(p);
        base
    } else {
        path
    };
    let config_path = config_path.canonicalize()?;
    let config_str = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&config_str)?)
}

fn post(url: &Url, msg: &str) -> Result<(), reqwest::Error> {
    let mut data = HashMap::new();
    data.insert("text", msg);

    Client::new()
        .post(url.as_str())
        .json(&data)
        .send()?
        .error_for_status()?;
    Ok(())
}

#[derive(Default)]
struct Limiter {
    previous: Option<Instant>,
}

impl Limiter {
    fn limit(&mut self) {
        if let Some(p) = self.previous {
            let since = p.elapsed();
            if since.as_millis() < u128::from(POST_INTERVAL) {
                let s = Duration::from_millis(POST_INTERVAL) - since;
                sleep(s);
            }
        }
        self.previous = Some(Instant::now());
    }
}
