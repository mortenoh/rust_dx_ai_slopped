//! CLI integration tests
//!
//! Tests all commands both with and without arguments.

use assert_cmd::Command;
use predicates::prelude::*;

fn dx() -> Command {
    Command::cargo_bin("dx").unwrap()
}

// ============================================================================
// Root command tests
// ============================================================================

#[test]
fn test_no_args_shows_help() {
    dx().assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_help_flag() {
    dx().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Developer Experience CLI"));
}

#[test]
fn test_version_flag() {
    dx().arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("dx"));
}

// ============================================================================
// Hash command tests
// ============================================================================

#[test]
fn test_hash_no_args_reads_stdin() {
    // hash without args reads from stdin
    dx().arg("hash").write_stdin("").assert().success();
}

#[test]
fn test_hash_help() {
    dx().args(["hash", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hash"));
}

#[test]
fn test_hash_string_md5() {
    dx().args(["hash", "-s", "hello", "-a", "md5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5d41402abc4b2a76b9719d911017c592"));
}

#[test]
fn test_hash_string_sha256() {
    dx().args(["hash", "-s", "hello", "-a", "sha256"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        ));
}

#[test]
fn test_hash_string_sha512() {
    dx().args(["hash", "-s", "hello", "-a", "sha512"])
        .assert()
        .success()
        .stdout(predicate::str::contains("9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"));
}

// ============================================================================
// Encode command tests
// ============================================================================

#[test]
fn test_encode_no_args_reads_stdin() {
    // encode without args reads from stdin
    dx().arg("encode").write_stdin("test").assert().success();
}

#[test]
fn test_encode_help() {
    dx().args(["encode", "--help"]).assert().success();
}

#[test]
fn test_encode_string_base64() {
    dx().args(["encode", "-s", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8="));
}

#[test]
fn test_encode_decode_base64() {
    dx().args(["encode", "-d", "-s", "aGVsbG8="])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_encode_hex() {
    dx().args(["encode", "-f", "hex", "-s", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("68656c6c6f"));
}

#[test]
fn test_encode_decode_hex() {
    dx().args(["encode", "-f", "hex", "-d", "-s", "68656c6c6f"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

// ============================================================================
// UUID command tests
// ============================================================================

#[test]
fn test_uuid_no_args_generates_uuid() {
    dx().arg("uuid").assert().success().stdout(
        predicate::str::is_match(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
            .unwrap(),
    );
}

#[test]
fn test_uuid_help() {
    dx().args(["uuid", "--help"]).assert().success();
}

#[test]
fn test_uuid_v4() {
    dx().args(["uuid", "-T", "v4"]).assert().success().stdout(
        predicate::str::is_match(
            r"[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}",
        )
        .unwrap(),
    );
}

#[test]
fn test_uuid_v7() {
    dx().args(["uuid", "-T", "v7"]).assert().success().stdout(
        predicate::str::is_match(
            r"[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}",
        )
        .unwrap(),
    );
}

#[test]
fn test_uuid_count() {
    dx().args(["uuid", "-c", "3"]).assert().success().stdout(
        predicate::str::is_match(r"(?s)[0-9a-f-]{36}.*[0-9a-f-]{36}.*[0-9a-f-]{36}").unwrap(),
    );
}

// ============================================================================
// Time command tests
// ============================================================================

#[test]
fn test_time_no_args() {
    dx().arg("time")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_time_help() {
    dx().args(["time", "--help"]).assert().success();
}

#[test]
fn test_time_now() {
    dx().args(["time", "now"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"\d{4}-\d{2}-\d{2}").unwrap());
}

#[test]
fn test_time_parse_unix() {
    dx().args(["time", "parse", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1970"));
}

#[test]
fn test_time_parse_rfc3339() {
    dx().args(["time", "parse", "2024-01-15T12:00:00Z"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2024"));
}

// ============================================================================
// JSON command tests
// ============================================================================

#[test]
fn test_json_no_args() {
    dx().arg("json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_json_help() {
    dx().args(["json", "--help"]).assert().success();
}

#[test]
fn test_json_format_stdin() {
    dx().args(["json", "format", "-"])
        .write_stdin(r#"{"a":1}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\""));
}

#[test]
fn test_json_validate_valid() {
    dx().args(["json", "validate", "-"])
        .write_stdin(r#"{"a":1}"#)
        .assert()
        .success();
}

#[test]
fn test_json_validate_invalid() {
    dx().args(["json", "validate", "-"])
        .write_stdin(r#"{invalid}"#)
        .assert()
        .failure();
}

#[test]
fn test_json_minify() {
    dx().args(["json", "minify", "-"])
        .write_stdin(r#"{ "a" : 1 }"#)
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"{"a":1}"#));
}

#[test]
fn test_json_query() {
    dx().args(["json", "query", "--path", ".name", "-"])
        .write_stdin(r#"{"name":"test"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("test"));
}

// ============================================================================
// Env command tests
// ============================================================================

#[test]
fn test_env_no_args() {
    dx().arg("env")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_env_help() {
    dx().args(["env", "--help"]).assert().success();
}

#[test]
fn test_env_get_path() {
    dx().args(["env", "get", "PATH"]).assert().success();
}

#[test]
fn test_env_get_nonexistent() {
    dx().args(["env", "get", "NONEXISTENT_VAR_12345"])
        .assert()
        .failure();
}

#[test]
fn test_env_list() {
    dx().args(["env", "list"]).assert().success();
}

// ============================================================================
// Config command tests
// ============================================================================

#[test]
fn test_config_no_args() {
    dx().arg("config")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_config_help() {
    dx().args(["config", "--help"]).assert().success();
}

#[test]
fn test_config_show() {
    dx().args(["config", "show"]).assert().success();
}

#[test]
fn test_config_path() {
    dx().args(["config", "path"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dx"));
}

// ============================================================================
// Rand command tests
// ============================================================================

#[test]
fn test_rand_no_args() {
    dx().arg("rand")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_rand_help() {
    dx().args(["rand", "--help"]).assert().success();
}

#[test]
fn test_rand_int() {
    dx().args(["rand", "int"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^-?\d+\n$").unwrap());
}

#[test]
fn test_rand_int_range() {
    dx().args(["rand", "int", "1", "100"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\d+\n$").unwrap());
}

#[test]
fn test_rand_float() {
    dx().args(["rand", "float"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^0\.\d+\n$").unwrap());
}

#[test]
fn test_rand_string() {
    dx().args(["rand", "string"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-zA-Z0-9]+\n$").unwrap());
}

#[test]
fn test_rand_string_length() {
    dx().args(["rand", "string", "8"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-zA-Z0-9]{8}\n$").unwrap());
}

#[test]
fn test_rand_hex() {
    dx().args(["rand", "hex"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]+\n$").unwrap());
}

#[test]
fn test_rand_hex_length() {
    dx().args(["rand", "hex", "8"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9a-f]{16}\n$").unwrap()); // 8 bytes = 16 hex chars
}

#[test]
fn test_rand_password() {
    dx().args(["rand", "password"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^.+\n$").unwrap());
}

#[test]
fn test_rand_password_length() {
    dx().args(["rand", "password", "20"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^.{20}\n$").unwrap());
}

#[test]
fn test_rand_choice() {
    dx().args(["rand", "choice", "a", "b", "c"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[abc]\n$").unwrap());
}

#[test]
fn test_rand_coin() {
    dx().args(["rand", "coin"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^(heads|tails)\n$").unwrap());
}

#[test]
fn test_rand_dice() {
    dx().args(["rand", "dice"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[1-6]\n$").unwrap());
}

// ============================================================================
// Text command tests
// ============================================================================

#[test]
fn test_text_no_args() {
    dx().arg("text")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_text_help() {
    dx().args(["text", "--help"]).assert().success();
}

#[test]
fn test_text_upper() {
    dx().args(["text", "upper", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO"));
}

#[test]
fn test_text_lower() {
    dx().args(["text", "lower", "HELLO"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_text_title() {
    dx().args(["text", "title", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello World"));
}

#[test]
fn test_text_snake() {
    dx().args(["text", "snake", "Hello World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello_world"));
}

#[test]
fn test_text_camel() {
    dx().args(["text", "camel", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("helloWorld"));
}

#[test]
fn test_text_pascal() {
    dx().args(["text", "pascal", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("HelloWorld"));
}

#[test]
fn test_text_kebab() {
    dx().args(["text", "kebab", "Hello World"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"));
}

#[test]
fn test_text_reverse() {
    dx().args(["text", "reverse", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("olleh"));
}

#[test]
fn test_text_slug() {
    dx().args(["text", "slug", "Hello World!"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"));
}

#[test]
fn test_text_count() {
    dx().args(["text", "count", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("chars"))
        .stdout(predicate::str::contains("words"));
}

#[test]
fn test_text_lorem() {
    dx().args(["text", "lorem"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Lorem"));
}

// ============================================================================
// Calc command tests
// ============================================================================

#[test]
fn test_calc_no_args() {
    dx().arg("calc")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_calc_help() {
    dx().args(["calc", "--help"]).assert().success();
}

#[test]
fn test_calc_bytes() {
    dx().args(["calc", "bytes", "1024"])
        .assert()
        .success()
        .stdout(predicate::str::contains("KB"));
}

#[test]
fn test_calc_bytes_gb() {
    dx().args(["calc", "bytes", "1gb"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bytes"));
}

#[test]
fn test_calc_time() {
    dx().args(["calc", "time", "3600"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1h"));
}

#[test]
fn test_calc_time_duration() {
    dx().args(["calc", "time", "1h30m"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5400"));
}

#[test]
fn test_calc_percent() {
    dx().args(["calc", "percent", "25", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("25"));
}

#[test]
fn test_calc_base_decimal_to_hex() {
    dx().args(["calc", "base", "255", "10", "16"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ff"));
}

#[test]
fn test_calc_base_binary_to_decimal() {
    dx().args(["calc", "base", "1010", "2", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("10"));
}

#[test]
fn test_calc_base_hex_to_binary() {
    dx().args(["calc", "base", "ff", "16", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("11111111"));
}

// ============================================================================
// Expr command tests
// ============================================================================

#[test]
fn test_expr_no_args() {
    dx().arg("expr")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_expr_help() {
    dx().args(["expr", "--help"]).assert().success();
}

#[test]
fn test_expr_eval_simple() {
    dx().args(["expr", "eval", "2 + 2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4"));
}

#[test]
fn test_expr_eval_precedence() {
    dx().args(["expr", "eval", "2 + 3 * 4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("14"));
}

#[test]
fn test_expr_eval_parentheses() {
    dx().args(["expr", "eval", "(2 + 3) * 4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("20"));
}

#[test]
fn test_expr_eval_power() {
    dx().args(["expr", "eval", "2^10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1024"));
}

#[test]
fn test_expr_eval_modulo() {
    dx().args(["expr", "eval", "17 % 5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn test_expr_eval_unary() {
    // Use -- to prevent -5 being interpreted as a flag
    dx().args(["expr", "eval", "--", "-5 + 10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_expr_eval_sqrt() {
    dx().args(["expr", "eval", "sqrt(16)"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4"));
}

#[test]
fn test_expr_eval_abs() {
    dx().args(["expr", "eval", "abs(-5)"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_expr_eval_pi() {
    dx().args(["expr", "eval", "pi"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3.14"));
}

#[test]
fn test_expr_eval_e() {
    dx().args(["expr", "eval", "e"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2.71"));
}

#[test]
fn test_expr_eval_complex() {
    dx().args(["expr", "eval", "sqrt(16) + pi"])
        .assert()
        .success()
        .stdout(predicate::str::contains("7.14"));
}

#[test]
fn test_expr_eval_trig() {
    dx().args(["expr", "eval", "sin(0)"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
}

#[test]
fn test_expr_eval_ln() {
    dx().args(["expr", "eval", "ln(e)"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1"));
}

#[test]
fn test_expr_ast() {
    dx().args(["expr", "ast", "2 + 3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("binop"));
}

#[test]
fn test_expr_ast_pretty() {
    dx().args(["expr", "ast", "2 + 3", "--pretty"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"binop\""));
}

#[test]
fn test_expr_list() {
    dx().args(["expr", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OPERATORS"))
        .stdout(predicate::str::contains("FUNCTIONS"))
        .stdout(predicate::str::contains("CONSTANTS"));
}

#[test]
fn test_expr_eval_error_unknown_function() {
    dx().args(["expr", "eval", "unknown(5)"]).assert().failure();
}

#[test]
fn test_expr_eval_error_division_by_zero() {
    dx().args(["expr", "eval", "1/0"]).assert().failure();
}

#[test]
fn test_expr_eval_error_sqrt_negative() {
    dx().args(["expr", "eval", "sqrt(-1)"]).assert().failure();
}

// ============================================================================
// Net command tests
// ============================================================================

#[test]
fn test_net_no_args() {
    dx().arg("net")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_net_help() {
    dx().args(["net", "--help"]).assert().success();
}

#[test]
fn test_net_ip() {
    dx().args(["net", "ip"]).assert().success();
}

// ============================================================================
// Chat command tests (just help, since it requires a server)
// ============================================================================

#[test]
fn test_chat_no_args() {
    dx().arg("chat")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_chat_help() {
    dx().args(["chat", "--help"]).assert().success();
}

// ============================================================================
// Alias tests
// ============================================================================

#[test]
fn test_alias_h_for_hash() {
    dx().args(["h", "-s", "test"]).assert().success();
}

#[test]
fn test_alias_e_for_encode() {
    dx().args(["e", "-s", "test"]).assert().success();
}

#[test]
fn test_alias_u_for_uuid() {
    dx().args(["u"]).assert().success();
}

#[test]
fn test_alias_t_for_time() {
    dx().args(["t", "now"]).assert().success();
}

#[test]
fn test_alias_j_for_json() {
    dx().args(["j", "format", "-"])
        .write_stdin("{}")
        .assert()
        .success();
}

#[test]
fn test_alias_r_for_rand() {
    dx().args(["r", "int"]).assert().success();
}

#[test]
fn test_alias_c_for_calc() {
    dx().args(["c", "bytes", "1024"]).assert().success();
}

#[test]
fn test_alias_x_for_expr() {
    dx().args(["x", "eval", "1+1"]).assert().success();
}

// ============================================================================
// Global flags tests
// ============================================================================

#[test]
fn test_no_color_flag() {
    dx().args(["--no-color", "expr", "list"]).assert().success();
}

#[test]
fn test_verbose_flag() {
    dx().args(["--verbose", "uuid"]).assert().success();
}

// ============================================================================
// Completions command tests
// ============================================================================

#[test]
fn test_completions_bash() {
    dx().args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_dx()"));
}

#[test]
fn test_completions_zsh() {
    dx().args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef dx"));
}

#[test]
fn test_completions_fish() {
    dx().args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -c dx"));
}

#[test]
fn test_completions_help() {
    dx().args(["completions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"))
        .stdout(predicate::str::contains("fish"));
}

// ============================================================================
// Fun command tests (help only, actual animations are timing-dependent)
// ============================================================================

#[test]
fn test_fun_no_args() {
    dx().arg("fun")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_fun_help() {
    dx().args(["fun", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("progress"))
        .stdout(predicate::str::contains("hacker"))
        .stdout(predicate::str::contains("countdown"))
        .stdout(predicate::str::contains("spinners"));
}

#[test]
fn test_fun_progress_help() {
    dx().args(["fun", "progress", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("duration"))
        .stdout(predicate::str::contains("style"));
}

#[test]
fn test_fun_hacker_help() {
    dx().args(["fun", "hacker", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("duration"))
        .stdout(predicate::str::contains("intensity"));
}

#[test]
fn test_fun_countdown_help() {
    dx().args(["fun", "countdown", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("seconds"))
        .stdout(predicate::str::contains("message"));
}

#[test]
fn test_fun_spinners_help() {
    dx().args(["fun", "spinners", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("duration"))
        .stdout(predicate::str::contains("name"));
}

#[test]
fn test_fun_work_help() {
    dx().args(["fun", "work", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("duration"))
        .stdout(predicate::str::contains("tasks"))
        .stdout(predicate::str::contains("style"));
}

#[test]
fn test_fun_work_list_styles() {
    dx().args(["fun", "work", "--list-styles"])
        .assert()
        .success()
        .stdout(predicate::str::contains("block"))
        .stdout(predicate::str::contains("gradient"))
        .stdout(predicate::str::contains("arrow"));
}

#[test]
fn test_fun_fortune_help() {
    dx().args(["fun", "fortune", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("animal"))
        .stdout(predicate::str::contains("say"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_fun_fortune_list() {
    dx().args(["fun", "fortune", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cow"))
        .stdout(predicate::str::contains("tux"))
        .stdout(predicate::str::contains("cat"));
}

// ============================================================================
// Grep command tests
// ============================================================================

#[test]
fn test_grep_no_args() {
    dx().arg("grep")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_grep_help() {
    dx().args(["grep", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PATTERN"))
        .stdout(predicate::str::contains("PATH"));
}

#[test]
fn test_grep_pattern_in_file() {
    // Search for a known pattern in our own source
    dx().args(["grep", "fn main", "src/main.rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fn main"));
}

#[test]
fn test_grep_case_insensitive() {
    dx().args(["grep", "-i", "FN MAIN", "src/main.rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fn main"));
}

#[test]
fn test_grep_with_context() {
    dx().args(["grep", "-C", "1", "fn main", "src/main.rs"])
        .assert()
        .success();
}

#[test]
fn test_grep_no_match() {
    dx().args(["grep", "NONEXISTENT_PATTERN_12345", "src/main.rs"])
        .assert()
        .failure(); // grep returns failure when no matches are found
}

#[test]
fn test_grep_alias_g() {
    dx().args(["g", "fn main", "src/main.rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fn main"));
}

// ============================================================================
// HTTP command tests
// ============================================================================

#[test]
fn test_http_no_args() {
    dx().arg("http")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_http_help() {
    dx().args(["http", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("post"));
}

#[test]
fn test_http_get_help() {
    dx().args(["http", "get", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("URL"));
}

#[test]
fn test_http_post_help() {
    dx().args(["http", "post", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("URL"))
        .stdout(predicate::str::contains("data"));
}

// Note: Actual HTTP requests are not tested to avoid network dependencies
// and flaky tests. The command structure is validated via help tests.

// ============================================================================
// Watch command tests
// ============================================================================

#[test]
fn test_watch_no_args() {
    dx().arg("watch")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_watch_help() {
    dx().args(["watch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PATH"))
        .stdout(predicate::str::contains("COMMAND"));
}

#[test]
fn test_watch_alias_w() {
    dx().args(["w", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Watch"));
}

// Note: Actual watch functionality requires running processes and is
// tested manually. The command structure is validated via help tests.

// ============================================================================
// System command tests
// ============================================================================

#[test]
fn test_system_no_args() {
    dx().arg("system")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_system_help() {
    dx().args(["system", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("info"))
        .stdout(predicate::str::contains("System information"));
}

#[test]
fn test_system_info() {
    dx().args(["system", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OS"))
        .stdout(predicate::str::contains("CPU"));
}

#[test]
fn test_system_info_shows_uptime() {
    dx().args(["system", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Uptime"));
}

#[test]
fn test_system_alias_sys() {
    dx().args(["sys", "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OS"));
}

// ============================================================================
// UI command tests (feature-gated)
// ============================================================================

#[test]
#[cfg(feature = "ui")]
fn test_ui_help() {
    dx().args(["ui", "--help"]).assert().success();
}

// Note: The TUI dashboard requires an interactive terminal and cannot be
// easily tested in CI. We only test that the command exists when the
// feature is enabled.

// ============================================================================
// Egui command tests (feature-gated)
// ============================================================================

#[test]
#[cfg(feature = "egui")]
fn test_egui_no_args() {
    dx().arg("egui")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
#[cfg(feature = "egui")]
fn test_egui_help() {
    dx().args(["egui", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("demo"))
        .stdout(predicate::str::contains("counter"))
        .stdout(predicate::str::contains("clock"));
}

#[test]
#[cfg(feature = "egui")]
fn test_egui_demo_help() {
    dx().args(["egui", "demo", "--help"]).assert().success();
}

#[test]
#[cfg(feature = "egui")]
fn test_egui_counter_help() {
    dx().args(["egui", "counter", "--help"]).assert().success();
}

#[test]
#[cfg(feature = "egui")]
fn test_egui_clock_help() {
    dx().args(["egui", "clock", "--help"]).assert().success();
}

// Note: Actual GUI windows cannot be tested in CI without a display.
// We only test that the commands parse correctly via help tests.
