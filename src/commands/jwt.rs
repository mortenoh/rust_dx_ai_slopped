//! JWT command - JSON Web Token utilities.

use crate::cli::commands::jwt::{DecodeFormat, JwtAlgorithm, JwtArgs, JwtCommand};
use anyhow::{Context, Result};
use colored::Colorize;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Run the jwt command
pub fn run(args: JwtArgs) -> Result<()> {
    match args.command {
        JwtCommand::Decode { token, format } => cmd_decode(&token, format),
        JwtCommand::Encode {
            secret,
            payload,
            algorithm,
            exp,
            sub,
            iss,
        } => cmd_encode(&secret, &payload, algorithm, exp, sub, iss),
        JwtCommand::Verify {
            token,
            secret,
            algorithm,
        } => cmd_verify(&token, &secret, algorithm),
    }
}

fn algorithm_to_jsonwebtoken(alg: JwtAlgorithm) -> Algorithm {
    match alg {
        JwtAlgorithm::Hs256 => Algorithm::HS256,
        JwtAlgorithm::Hs384 => Algorithm::HS384,
        JwtAlgorithm::Hs512 => Algorithm::HS512,
    }
}

fn cmd_decode(token: &str, format: DecodeFormat) -> Result<()> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid JWT format: expected 3 parts separated by '.'");
    }

    // Decode header (first part)
    let header_json = base64_decode_url_safe(parts[0]).context("Failed to decode header")?;
    let header: Value = serde_json::from_slice(&header_json).context("Failed to parse header")?;

    // Decode payload (second part)
    let payload_json = base64_decode_url_safe(parts[1]).context("Failed to decode payload")?;
    let payload: Value =
        serde_json::from_slice(&payload_json).context("Failed to parse payload")?;

    match format {
        DecodeFormat::Pretty => {
            println!("{}", "Header:".cyan().bold());
            println!("{}", serde_json::to_string_pretty(&header)?);
            println!("\n{}", "Payload:".cyan().bold());
            println!("{}", serde_json::to_string_pretty(&payload)?);
            println!("\n{}", "Signature:".cyan().bold());
            println!("{}", parts[2]);
        }
        DecodeFormat::Json => {
            let mut output = Map::new();
            output.insert("header".to_string(), header);
            output.insert("payload".to_string(), payload);
            output.insert("signature".to_string(), Value::String(parts[2].to_string()));
            println!("{}", serde_json::to_string(&Value::Object(output))?);
        }
        DecodeFormat::Raw => {
            println!("Header (base64): {}", parts[0]);
            println!("Payload (base64): {}", parts[1]);
            println!("Signature (base64): {}", parts[2]);
        }
    }

    Ok(())
}

fn base64_decode_url_safe(input: &str) -> Result<Vec<u8>> {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    URL_SAFE_NO_PAD
        .decode(input)
        .context("Invalid base64 encoding")
}

fn cmd_encode(
    secret: &str,
    payload: &str,
    algorithm: JwtAlgorithm,
    exp: Option<i64>,
    sub: Option<String>,
    iss: Option<String>,
) -> Result<()> {
    // Parse the payload JSON
    let mut claims: HashMap<String, Value> =
        serde_json::from_str(payload).context("Failed to parse payload JSON")?;

    // Add standard claims if provided
    if let Some(exp_secs) = exp {
        let exp_time = chrono::Utc::now().timestamp() + exp_secs;
        claims.insert("exp".to_string(), Value::Number(exp_time.into()));
    }

    if let Some(sub_val) = sub {
        claims.insert("sub".to_string(), Value::String(sub_val));
    }

    if let Some(iss_val) = iss {
        claims.insert("iss".to_string(), Value::String(iss_val));
    }

    // Add issued-at time
    claims.insert(
        "iat".to_string(),
        Value::Number(chrono::Utc::now().timestamp().into()),
    );

    let header = Header::new(algorithm_to_jsonwebtoken(algorithm));
    let key = EncodingKey::from_secret(secret.as_bytes());

    let token = encode(&header, &claims, &key).context("Failed to encode JWT")?;

    println!("{}", token);
    Ok(())
}

fn cmd_verify(token: &str, secret: &str, algorithm: JwtAlgorithm) -> Result<()> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(algorithm_to_jsonwebtoken(algorithm));
    validation.validate_exp = true;
    validation.required_spec_claims.clear(); // Don't require any specific claims

    match decode::<HashMap<String, Value>>(token, &key, &validation) {
        Ok(token_data) => {
            println!("{}", "Token is valid!".green().bold());
            println!("\n{}", "Claims:".cyan().bold());
            println!("{}", serde_json::to_string_pretty(&token_data.claims)?);
            Ok(())
        }
        Err(e) => {
            eprintln!("{}: {}", "Token verification failed".red().bold(), e);
            anyhow::bail!("Invalid token")
        }
    }
}
