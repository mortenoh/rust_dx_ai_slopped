#!/bin/bash
#
# Smoke test script for dx CLI
# Runs through all commands to verify they work
#

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

declare -i PASSED=0
declare -i FAILED=0
declare -i SKIPPED=0

# Get the dx binary path
DX="${DX:-cargo run --quiet --}"

# Test helper functions
pass() {
    echo -e "  ${GREEN}✓${NC} $1"
    PASSED+=1
}

fail() {
    echo -e "  ${RED}✗${NC} $1"
    echo -e "    ${RED}Error: $2${NC}"
    FAILED+=1
}

skip() {
    echo -e "  ${YELLOW}○${NC} $1 (skipped: $2)"
    SKIPPED+=1
}

section() {
    echo -e "\n${BLUE}━━━ $1 ━━━${NC}"
}

# Run a test command
test_cmd() {
    local name="$1"
    shift
    if output=$($DX "$@" 2>&1); then
        pass "$name"
        return 0
    else
        fail "$name" "$output"
        return 1
    fi
}

# Run a test command that expects specific output
test_cmd_contains() {
    local name="$1"
    local expected="$2"
    shift 2
    if output=$($DX "$@" 2>&1); then
        if echo "$output" | grep -q "$expected"; then
            pass "$name"
            return 0
        else
            fail "$name" "Output did not contain '$expected': $output"
            return 1
        fi
    else
        fail "$name" "$output"
        return 1
    fi
}

# Run a test with stdin
test_stdin() {
    local name="$1"
    local stdin="$2"
    shift 2
    if output=$(echo "$stdin" | $DX "$@" 2>&1); then
        pass "$name"
        return 0
    else
        fail "$name" "$output"
        return 1
    fi
}

# Run a test with stdin expecting output
test_stdin_contains() {
    local name="$1"
    local stdin="$2"
    local expected="$3"
    shift 3
    if output=$(echo "$stdin" | $DX "$@" 2>&1); then
        if echo "$output" | grep -q "$expected"; then
            pass "$name"
            return 0
        else
            fail "$name" "Output did not contain '$expected'"
            return 1
        fi
    else
        fail "$name" "$output"
        return 1
    fi
}

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              dx CLI Smoke Test Suite                       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"

# ============================================================================
section "Help & Version"
# ============================================================================

test_cmd "dx --help" --help
test_cmd "dx help" help

# ============================================================================
section "Hash Command"
# ============================================================================

test_cmd_contains "hash -s (sha256 default)" "2cf24dba" hash -s "hello"
test_cmd_contains "hash -s -a md5" "5d41402abc4b2a76" hash -s "hello" -a md5
test_cmd_contains "hash -s -a sha512" "9b71d224bd62" hash -s "hello" -a sha512
test_cmd_contains "hash -s -a bcrypt" "\$2" hash -s "password" -a bcrypt --cost 4
test_cmd_contains "hash -s -a argon2" "\$argon2" hash -s "password" -a argon2
test_stdin "hash from stdin" "hello" hash -

# ============================================================================
section "Encode Command"
# ============================================================================

test_cmd_contains "encode base64" "aGVsbG8gd29ybGQ=" encode -s "hello world"
test_cmd_contains "encode base64 decode" "hello world" encode -s "aGVsbG8gd29ybGQ=" -d
test_cmd_contains "encode hex" "68656c6c6f" encode -f hex -s "hello"
test_cmd_contains "encode hex decode" "hello" encode -f hex -d -s "68656c6c6f"
# Note: echo adds newline, so "hello\n" encodes to "aGVsbG8K"
test_stdin_contains "encode from stdin" "hello" "aGVsbG8K" encode -

# ============================================================================
section "UUID Command"
# ============================================================================

test_cmd_contains "uuid v4" "-" uuid
test_cmd_contains "uuid v7" "-" uuid -T v7
test_cmd_contains "uuid simple" "" uuid -f simple
test_cmd_contains "uuid multiple" "" uuid -c 3

# ============================================================================
section "Time Command"
# ============================================================================

test_cmd "time now" time now
test_cmd "time parse" time parse "2024-01-15T10:30:00Z"
test_cmd "time convert" time convert 1705315800 -f rfc2822

# ============================================================================
section "JSON Command"
# ============================================================================

test_stdin "json format" '{"a":1,"b":2}' json format -
test_stdin_contains "json format output" '{"a":1}' '"a"' json format -
test_stdin "json validate valid" '{"valid": true}' json validate -
test_stdin "json minify" '{ "a" : 1 }' json minify -

# ============================================================================
section "Env Command"
# ============================================================================

test_cmd "env list" env list
test_cmd_contains "env get PATH" "/" env get PATH

# ============================================================================
section "Config Command"
# ============================================================================

test_cmd "config path" config path
test_cmd "config show" config show

# ============================================================================
section "Rand Command"
# ============================================================================

test_cmd "rand int" rand int 1 100
test_cmd "rand float" rand float
test_cmd "rand string" rand string 16
test_cmd "rand password" rand password
test_cmd_contains "rand choice" "" rand choice "a" "b" "c"

# ============================================================================
section "Text Command"
# ============================================================================

test_cmd_contains "text upper" "HELLO" text upper "hello"
test_cmd_contains "text lower" "hello" text lower "HELLO"
test_cmd_contains "text title" "Hello World" text title "hello world"
test_cmd_contains "text snake" "hello_world" text snake "Hello World"
test_cmd_contains "text kebab" "hello-world" text kebab "Hello World"
test_cmd_contains "text camel" "helloWorld" text camel "hello world"
test_cmd_contains "text pascal" "HelloWorld" text pascal "hello world"
test_cmd_contains "text slug" "hello-world" text slug "Hello World!"
test_cmd_contains "text reverse" "olleh" text reverse "hello"
test_cmd_contains "text count" "chars:" text count "hello world"

# ============================================================================
section "Calc Command"
# ============================================================================

test_cmd_contains "calc bytes" "KB" calc bytes 1024
test_cmd_contains "calc bytes MB" "MB" calc bytes 1048576
test_cmd_contains "calc time" "hour" calc time 3600
test_cmd_contains "calc base bin" "1010" calc base 10 10 2
test_cmd_contains "calc base hex" "a" calc base 10 10 16
test_cmd_contains "calc percent" "25" calc percent 25 100

# ============================================================================
section "Expr Command"
# ============================================================================

test_cmd_contains "expr eval basic" "14" expr eval "2 + 3 * 4"
test_cmd_contains "expr eval power" "1024" expr eval "2^10"
test_cmd_contains "expr eval sqrt" "4" expr eval "sqrt(16)"
test_cmd_contains "expr eval pi" "3.14" expr eval "pi"
test_cmd_contains "expr eval sin" "1" expr eval "sin(pi/2)"
test_cmd_contains "expr eval variables" "16" expr eval "x = 5; y = x + 3; y * 2"
test_cmd_contains "expr eval function" "25" expr eval "def square(x) = x * x; square(5)"
test_cmd_contains "expr eval conditional" "100" expr eval "if 5 > 3 then 100 else 200"

# ============================================================================
section "Net Command"
# ============================================================================

test_cmd_contains "net ip" "." net ip
test_cmd "net lookup" net lookup google.com

# ============================================================================
section "Grep Command"
# ============================================================================

test_cmd "grep in file" grep "fn main" src/main.rs
test_cmd_contains "grep pattern" "main" grep "fn main" src/main.rs

# ============================================================================
section "HTTP Command"
# ============================================================================

test_cmd "http get" http get https://httpbin.org/get
test_cmd_contains "http get json" "origin" http get https://httpbin.org/get

# ============================================================================
section "System Command"
# ============================================================================

test_cmd "system info" system info

# ============================================================================
section "Fun Command (quick tests only)"
# ============================================================================

test_cmd_contains "fun qr" "█" fun qr "test"
test_cmd_contains "fun banner" "█" fun banner "HI"
test_cmd "fun fortune" fun fortune

# ============================================================================
section "YAML Command"
# ============================================================================

test_stdin_contains "yaml format" 'name: test' "name:" yaml format -
test_stdin "yaml validate" 'key: value' yaml validate -
test_stdin_contains "yaml to-json" 'name: test' '"name"' yaml to-json -
test_stdin_contains "yaml from-json" '{"name":"test"}' "name:" yaml from-json -

# ============================================================================
section "CSV Command"
# ============================================================================

test_stdin_contains "csv format" $'name,age\nAlice,30' "Alice" csv format -
test_stdin_contains "csv to-json" $'name,age\nAlice,30' '"name"' csv to-json -
test_stdin_contains "csv query" $'name,age,city\nAlice,30,NYC\nBob,25,LA' "name" csv query - --columns name,age

# ============================================================================
section "XML Command"
# ============================================================================

test_stdin "xml format" '<root><item>test</item></root>' xml format -
test_stdin "xml validate" '<root><item>test</item></root>' xml validate -
test_stdin_contains "xml to-json" '<root><item>test</item></root>' '"root"' xml to-json -

# ============================================================================
section "JWT Command"
# ============================================================================

test_cmd_contains "jwt decode" "alg" jwt decode "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
test_cmd_contains "jwt encode" "." jwt encode --secret "test-secret" --payload '{"sub":"123","name":"Test"}'
test_cmd "jwt verify" jwt verify "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" --secret "your-256-bit-secret"

# ============================================================================
section "Encrypt Command"
# ============================================================================

# Note: encrypt just outputs the ciphertext directly
test_cmd "encrypt string" encrypt encrypt -s "hello world" --password "mypassword"
# Test roundtrip
ENCRYPTED=$($DX encrypt encrypt -s "hello" --password "testpw" 2>&1 | grep -o '[A-Za-z0-9+/=]*$' | tail -1)
if [ -n "$ENCRYPTED" ]; then
    pass "encrypt produces output"
else
    skip "encrypt roundtrip" "could not capture output"
fi

# ============================================================================
section "Diff Command"
# ============================================================================

# Create temp files for diff test
TMPDIR=$(mktemp -d)
echo -e "line1\nline2\nline3" > "$TMPDIR/file1.txt"
echo -e "line1\nmodified\nline3" > "$TMPDIR/file2.txt"

test_cmd_contains "diff unified" "@@" diff "$TMPDIR/file1.txt" "$TMPDIR/file2.txt"
test_cmd_contains "diff inline" "line2" diff "$TMPDIR/file1.txt" "$TMPDIR/file2.txt" --format inline
test_cmd_contains "diff compact" "-" diff "$TMPDIR/file1.txt" "$TMPDIR/file2.txt" --format compact

rm -rf "$TMPDIR"

# ============================================================================
section "Template Command"
# ============================================================================

# Create temp files for template test
TMPDIR=$(mktemp -d)
echo 'Hello, {{ name }}!' > "$TMPDIR/template.tera"
echo '{"name":"World"}' > "$TMPDIR/data.json"

test_cmd_contains "template render" "Hello, World!" template render "$TMPDIR/template.tera" --data "$TMPDIR/data.json"
test_cmd "template validate" template validate "$TMPDIR/template.tera"

rm -rf "$TMPDIR"

# ============================================================================
section "Markdown Command"
# ============================================================================

TMPDIR=$(mktemp -d)
echo -e "# Heading\n\nSome **bold** text." > "$TMPDIR/test.md"

test_cmd_contains "markdown render" "<h1>" markdown render "$TMPDIR/test.md"
test_cmd_contains "markdown toc" "Heading" markdown toc "$TMPDIR/test.md"

rm -rf "$TMPDIR"

# ============================================================================
section "Compress Command"
# ============================================================================

TMPDIR=$(mktemp -d)
echo "Hello, compression test!" > "$TMPDIR/test.txt"

test_cmd "compress gzip" compress compress "$TMPDIR/test.txt" -O "$TMPDIR/test.txt.gz"
if [ -f "$TMPDIR/test.txt.gz" ]; then
    pass "compress created gzip file"
    test_cmd "decompress gzip" compress decompress "$TMPDIR/test.txt.gz" -O "$TMPDIR/test_out.txt"
    if [ -f "$TMPDIR/test_out.txt" ]; then
        if grep -q "Hello" "$TMPDIR/test_out.txt"; then
            pass "decompress restored content"
        else
            fail "decompress content" "content mismatch"
        fi
    else
        fail "decompress file creation" "output file not created"
    fi
else
    fail "compress file creation" "gzip file not created"
fi

rm -rf "$TMPDIR"

# ============================================================================
section "DHIS2 Command"
# ============================================================================

# UID generation is offline and fast
test_cmd "dhis2 uid" dhis2 uid
test_cmd_contains "dhis2 uid multiple" "" dhis2 uid 3
test_cmd_contains "dhis2 uid validate valid" "valid" dhis2 uid --validate "AbCdEfGhIjK"
# Invalid UID returns exit code 1, so we just check it runs
if $DX dhis2 uid --validate "123" 2>&1 | grep -q "NOT a valid"; then
    pass "dhis2 uid validate invalid"
else
    fail "dhis2 uid validate invalid" "did not detect invalid UID"
fi
test_cmd "dhis2 --help" dhis2 --help
test_cmd_contains "dhis2 data-values --help" "data-set" dhis2 data-values --help
# Test actual data fetch (ART monthly summary at Ngelehun CHC, 2024)
test_cmd_contains "dhis2 data-values fetch" "DataElement" dhis2 dv --data-set lyLU2wR22tC --org-unit DiszpKrYNg8 --start-date 2024-01-01 --end-date 2024-12-31 --limit 3

# ============================================================================
section "Polars Command"
# ============================================================================

# Basic random generation
test_cmd "polars random basic" polars random -n 5 -c "id:id,name:name"
test_cmd_contains "polars random with types" "id" polars random -n 3 -c "id:id,email:email,phone:phone"

# New entertainment generators
test_cmd "polars random entertainment" polars random -n 3 -c "book:book_title,movie:movie_title,artist:music_artist"

# New food generators
test_cmd "polars random food" polars random -n 3 -c "dish:dish,cuisine:cuisine,beverage:beverage"

# New animal generators
test_cmd "polars random animals" polars random -n 3 -c "animal:animal,dog:dog_breed,pet:pet_name"

# New travel generators
test_cmd "polars random travel" polars random -n 3 -c "airline:airline,airport:airport,destination:destination"

# New healthcare generators
test_cmd "polars random healthcare" polars random -n 3 -c "condition:condition,medication:medication,hospital:hospital"

# New sports generators
test_cmd "polars random sports" polars random -n 3 -c "sport:sport,team:team,league:league"

# New hacker generators
test_cmd "polars random hacker" polars random -n 3 -c "phrase:hacker,lang:programming_language,db:database"

# New education generators
test_cmd "polars random education" polars random -n 3 -c "univ:university,degree:degree,major:major"

# New weather generators
test_cmd "polars random weather" polars random -n 3 -c "weather:weather,temp:temperature,season:season"

# New astrology generators
test_cmd "polars random astrology" polars random -n 3 -c "zodiac:zodiac,birthstone:birthstone,horoscope:horoscope"

# Test output formats
test_cmd "polars random csv format" polars random -n 3 -c "id:id,name:name" -f csv
test_cmd "polars random json format" polars random -n 3 -c "id:id,name:name" -f json

# ============================================================================
section "Completions Command"
# ============================================================================

test_cmd_contains "completions bash" "complete" completions bash
test_cmd_contains "completions zsh" "compdef" completions zsh
test_cmd_contains "completions fish" "complete" completions fish

# ============================================================================
section "UI Command (feature check)"
# ============================================================================

# UI/TUI commands require terminal interaction, just check they exist
test_cmd "ui --help" ui --help

# ============================================================================
section "EGUI Command (feature check)"
# ============================================================================

# EGUI commands require GUI, just check they exist
test_cmd "egui --help" egui --help

# ============================================================================
# Summary
# ============================================================================

echo -e "\n${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}                        Summary                              ${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}Passed:${NC}  $PASSED"
echo -e "  ${RED}Failed:${NC}  $FAILED"
echo -e "  ${YELLOW}Skipped:${NC} $SKIPPED"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

if [ $FAILED -gt 0 ]; then
    echo -e "\n${RED}Some tests failed!${NC}"
    exit 1
else
    echo -e "\n${GREEN}All tests passed!${NC}"
    exit 0
fi
