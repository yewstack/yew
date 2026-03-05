use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use tokio::process::Command;
use tokio::time::{sleep, Instant};

#[derive(Parser)]
struct Args {
    /// Shell command to start the SSR server
    #[clap(long)]
    server_cmd: String,

    /// URL to poll until the server is ready (expects HTTP 200)
    #[clap(long)]
    health_url: String,

    /// Cargo package name whose tests to run
    #[clap(long)]
    test_pkg: String,

    /// Directory containing a Trunk project (index.html).
    /// If provided, `trunk build` is run here before starting the server.
    #[clap(long)]
    trunk_dir: Option<String>,

    /// Server port, exposed as SSR_TEST_PORT env var
    #[clap(long, default_value_t = 8080)]
    port: u16,

    /// Max seconds to wait for the server to become ready
    #[clap(long, default_value_t = 120)]
    timeout: u64,

    /// Extra arguments passed to `cargo test`
    #[clap(last = true)]
    extra_args: Vec<String>,
}

async fn run_trunk_build(dir: &str) -> bool {
    eprintln!("[ssr-e2e] Running trunk build in {dir} ...");

    let status = Command::new("trunk")
        .args(["build"])
        .current_dir(dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await;

    match status {
        Ok(s) if s.success() => {
            eprintln!("[ssr-e2e] trunk build succeeded.");
            true
        }
        Ok(s) => {
            eprintln!(
                "[ssr-e2e] trunk build failed with exit code: {}",
                s.code().unwrap_or(-1)
            );
            false
        }
        Err(e) => {
            eprintln!("[ssr-e2e] Failed to run trunk build: {e}");
            false
        }
    }
}

async fn wait_for_server(url: &str, timeout: Duration) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    let start = Instant::now();
    let mut interval = Duration::from_millis(100);

    while start.elapsed() < timeout {
        if let Ok(resp) = client.get(url).send().await {
            if resp.status().is_success() {
                return true;
            }
        }
        sleep(interval).await;
        interval = (interval * 2).min(Duration::from_secs(2));
    }
    false
}

#[cfg(unix)]
fn kill_process_group(id: u32) {
    unsafe {
        libc::kill(-(id as i32), libc::SIGTERM);
    }
}

#[cfg(not(unix))]
fn kill_process_group(_id: u32) {}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    if let Some(ref trunk_dir) = args.trunk_dir {
        if !run_trunk_build(trunk_dir).await {
            return ExitCode::FAILURE;
        }
    }

    eprintln!("[ssr-e2e] Starting server: {}", args.server_cmd);

    let mut server = {
        let mut cmd = Command::new("sh");
        cmd.args(["-c", &args.server_cmd]);
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());
        #[cfg(unix)]
        unsafe {
            cmd.pre_exec(|| {
                libc::setpgid(0, 0);
                Ok(())
            });
        }
        cmd.spawn().expect("failed to start server process")
    };

    let server_pid = server.id().expect("server has no pid");

    eprintln!("[ssr-e2e] Waiting for server at {} ...", args.health_url);

    let ready = wait_for_server(&args.health_url, Duration::from_secs(args.timeout)).await;
    if !ready {
        eprintln!(
            "[ssr-e2e] Server did not become ready within {}s",
            args.timeout
        );
        kill_process_group(server_pid);
        let _ = server.wait().await;
        return ExitCode::FAILURE;
    }

    eprintln!(
        "[ssr-e2e] Server is ready. Running tests for {} ...",
        args.test_pkg
    );

    let mut cargo_args = vec!["test".to_string(), "-p".to_string(), args.test_pkg.clone()];
    cargo_args.extend(args.extra_args);

    let test_result = Command::new("cargo")
        .args(&cargo_args)
        .env("SSR_TEST_URL", format!("http://127.0.0.1:{}", args.port))
        .env("SSR_TEST_PORT", args.port.to_string())
        .env("WASM_BINDGEN_TEST_NO_ORIGIN_ISOLATION", "1")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await;

    eprintln!("[ssr-e2e] Shutting down server ...");
    kill_process_group(server_pid);
    let _ = server.wait().await;

    match test_result {
        Ok(status) if status.success() => {
            eprintln!("[ssr-e2e] Tests passed.");
            ExitCode::SUCCESS
        }
        Ok(status) => {
            eprintln!(
                "[ssr-e2e] Tests failed with exit code: {}",
                status.code().unwrap_or(-1)
            );
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("[ssr-e2e] Failed to run tests: {e}");
            ExitCode::FAILURE
        }
    }
}
