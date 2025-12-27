# Maintenance

Keep your CLI healthy long-term.

## Dependency Updates

### cargo-outdated

```bash
cargo install cargo-outdated
cargo outdated
```

### Dependabot

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    groups:
      rust-dependencies:
        patterns:
          - "*"
```

### Renovate

```json
// renovate.json
{
  "extends": ["config:base"],
  "packageRules": [
    {
      "matchManagers": ["cargo"],
      "groupName": "rust dependencies"
    }
  ]
}
```

## Security Audits

### cargo-audit

```bash
cargo install cargo-audit
cargo audit
```

### CI Integration

```yaml
- name: Security audit
  run: |
    cargo install cargo-audit
    cargo audit
```

## Issue Management

### Issue Templates

```markdown
<!-- .github/ISSUE_TEMPLATE/bug_report.md -->
---
name: Bug report
about: Report a bug
---

**Description**
A clear description of the bug.

**To Reproduce**
1. Run `dx ...`
2. See error

**Expected behavior**
What should happen.

**Environment**
- OS: [e.g., macOS 14.0]
- dx version: [e.g., 1.2.3]
```

### Labels

- `bug` - Something isn't working
- `enhancement` - New feature request
- `documentation` - Documentation improvements
- `good first issue` - Good for newcomers

## Pull Request Template

```markdown
<!-- .github/pull_request_template.md -->
## Description
What does this PR do?

## Testing
How was this tested?

## Checklist
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] CHANGELOG updated
```

## CI Checks

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Test
        run: cargo test

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Format
        run: cargo fmt -- --check

      - name: Audit
        run: |
          cargo install cargo-audit
          cargo audit
```

## Deprecation Policy

```rust
// 1. Add deprecation warning
#[deprecated(since = "1.3.0", note = "Use `new_command` instead")]
pub fn old_command() {}

// 2. Document in CHANGELOG
// ## [1.3.0]
// ### Deprecated
// - `old_command` is deprecated, use `new_command`

// 3. Remove in next major version
// ## [2.0.0]
// ### Removed
// - Removed deprecated `old_command`
```

## Telemetry (Optional)

```rust
// Only with explicit opt-in
fn send_anonymous_usage() {
    if !config.telemetry_enabled {
        return;
    }

    // Send only:
    // - Command used (not arguments)
    // - OS and architecture
    // - Version
}
```

## Support Channels

- GitHub Issues - Bug reports and features
- GitHub Discussions - Questions and ideas
- Discord/Slack - Community chat
- Email - Security issues

## Release Checklist

1. [ ] All tests passing
2. [ ] CHANGELOG updated
3. [ ] Version bumped
4. [ ] Documentation updated
5. [ ] Security audit clean
6. [ ] Release notes drafted
7. [ ] Tag created
8. [ ] Binaries built
9. [ ] Packages published
10. [ ] Announcement posted
