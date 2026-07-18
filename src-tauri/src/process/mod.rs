use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use thiserror::Error;
use tokio::process::Command;
use tokio::time;

#[derive(Debug, Clone)]
pub struct ProcessRequest {
    pub executable: PathBuf,
    pub arguments: Vec<OsString>,
    pub environment: HashMap<OsString, OsString>,
    pub timeout: Duration,
    pub max_output_bytes: usize,
    pub requires_tty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub exit_code: Option<i32>,
    pub success: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProcessError {
    #[error("executable path must be absolute")]
    ExecutableNotAbsolute,
    #[error("pty execution is not available in M4")]
    PtyUnsupported,
    #[error("process timed out")]
    TimedOut,
    #[error("failed to run process")]
    SpawnFailed,
}

#[async_trait]
pub trait ProcessRunner: Send + Sync {
    async fn run(&self, request: ProcessRequest) -> Result<ProcessOutput, ProcessError>;
}

#[derive(Debug, Clone, Copy)]
pub struct TokioProcessRunner;

#[async_trait]
impl ProcessRunner for TokioProcessRunner {
    async fn run(&self, request: ProcessRequest) -> Result<ProcessOutput, ProcessError> {
        if !request.executable.is_absolute() {
            return Err(ProcessError::ExecutableNotAbsolute);
        }
        if request.requires_tty {
            return Err(ProcessError::PtyUnsupported);
        }

        let mut command = Command::new(&request.executable);
        command.args(&request.arguments);
        command.env_clear();
        for (key, value) in &request.environment {
            command.env(key, value);
        }

        let child = command.output();
        let output = time::timeout(request.timeout, child)
            .await
            .map_err(|_| ProcessError::TimedOut)?
            .map_err(|_| ProcessError::SpawnFailed)?;

        let (stdout, stdout_truncated) = cap_bytes(output.stdout, request.max_output_bytes);
        let (stderr, stderr_truncated) = cap_bytes(output.stderr, request.max_output_bytes);

        Ok(ProcessOutput {
            stdout,
            stderr,
            stdout_truncated,
            stderr_truncated,
            exit_code: output.status.code(),
            success: output.status.success(),
        })
    }
}

fn cap_bytes(bytes: Vec<u8>, max_bytes: usize) -> (Vec<u8>, bool) {
    if bytes.len() <= max_bytes {
        return (bytes, false);
    }

    (bytes.into_iter().take(max_bytes).collect(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::time::Duration;

    #[tokio::test]
    async fn rejects_relative_executables() {
        let runner = TokioProcessRunner;
        let request = ProcessRequest {
            executable: PathBuf::from("echo"),
            arguments: vec![OsString::from("hello")],
            environment: Default::default(),
            timeout: Duration::from_secs(1),
            max_output_bytes: 1024,
            requires_tty: false,
        };

        let result = runner.run(request).await;

        assert!(matches!(result, Err(ProcessError::ExecutableNotAbsolute)));
    }

    #[tokio::test]
    async fn preserves_exit_code() {
        let runner = TokioProcessRunner;
        let request = ProcessRequest {
            executable: PathBuf::from("/bin/sh"),
            arguments: vec![OsString::from("-c"), OsString::from("exit 7")],
            environment: Default::default(),
            timeout: Duration::from_secs(2),
            max_output_bytes: 1024,
            requires_tty: false,
        };

        let output = runner.run(request).await.expect("process output");

        assert_eq!(output.exit_code, Some(7));
        assert!(!output.success);
    }

    #[tokio::test]
    async fn caps_stdout() {
        let runner = TokioProcessRunner;
        let request = ProcessRequest {
            executable: PathBuf::from("/bin/sh"),
            arguments: vec![OsString::from("-c"), OsString::from("printf 1234567890")],
            environment: Default::default(),
            timeout: Duration::from_secs(2),
            max_output_bytes: 4,
            requires_tty: false,
        };

        let output = runner.run(request).await.expect("process output");

        assert_eq!(output.stdout, b"1234");
        assert!(output.stdout_truncated);
    }

    #[tokio::test]
    async fn times_out_long_running_process() {
        let runner = TokioProcessRunner;
        let request = ProcessRequest {
            executable: PathBuf::from("/bin/sh"),
            arguments: vec![OsString::from("-c"), OsString::from("sleep 1")],
            environment: Default::default(),
            timeout: Duration::from_millis(10),
            max_output_bytes: 1024,
            requires_tty: false,
        };

        let result = runner.run(request).await;

        assert!(matches!(result, Err(ProcessError::TimedOut)));
    }
}
