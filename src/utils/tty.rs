pub fn stderr_isatty() -> bool {
    atty::is(atty::Stream::Stderr)
}

pub fn stdout_isatty() -> bool {
    atty::is(atty::Stream::Stdout)
}
