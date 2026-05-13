//! Secret detection patterns

/// Built-in secret patterns: (name, regex_pattern, description)
/// Note: Using regular strings with escaped backslashes for patterns containing quotes
pub const BUILTIN_PATTERNS: &[(&str, &str, &str)] = &[
    (
        "aws_access_key",
        r"AKIA[0-9A-Z]{16}",
        "AWS Access Key ID",
    ),
    (
        "aws_secret_key",
        "(?i)aws[_-]?secret[_-]?access[_-]?key\\s*[:=]\\s*[\"'][a-zA-Z0-9/+=]{40}[\"']",
        "AWS Secret Access Key",
    ),
    (
        "github_token",
        r"gh[pousr]_[A-Za-z0-9_]{36,}",
        "GitHub Personal Access Token",
    ),
    (
        "github_oauth",
        r"gho_[A-Za-z0-9_]{36,}",
        "GitHub OAuth Access Token",
    ),
    (
        "private_key",
        r"-----BEGIN (RSA |EC |DSA |OPENSSH )?PRIVATE KEY-----",
        "Private Key",
    ),
    (
        "api_key_generic",
        "(?i)(api[_-]?key|apikey)\\s*[:=]\\s*[\"'][a-zA-Z0-9_\\-]{32,}[\"']",
        "Generic API Key",
    ),
    (
        "slack_token",
        r"xox[bprs]-[A-Za-z0-9-]+",
        "Slack Token",
    ),
    (
        "slack_webhook",
        r"https://hooks\.slack\.com/services/T[a-zA-Z0-9_]+/B[a-zA-Z0-9_]+/[a-zA-Z0-9_]+",
        "Slack Webhook URL",
    ),
    (
        "google_api_key",
        r"AIza[0-9A-Za-z\-_]{35}",
        "Google API Key",
    ),
    (
        "heroku_api_key",
        "(?i)heroku[_-]?api[_-]?key\\s*[:=]\\s*[\"'][a-f0-9]{32}[\"']",
        "Heroku API Key",
    ),
];
