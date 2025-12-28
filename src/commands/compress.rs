//! Compress command - compression utilities.

use crate::cli::commands::compress::{CompressArgs, CompressCommand, CompressionFormat};
use anyhow::{Context, Result};
use colored::Colorize;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

/// Run the compress command
pub fn run(args: CompressArgs) -> Result<()> {
    match args.command {
        CompressCommand::Compress {
            input,
            out_file,
            format,
            level,
        } => cmd_compress(&input, out_file, format, level),
        CompressCommand::Decompress { input, out_file } => cmd_decompress(&input, out_file),
    }
}

fn cmd_compress(
    input: &PathBuf,
    out_file: Option<PathBuf>,
    format: CompressionFormat,
    level: u32,
) -> Result<()> {
    let output_path = out_file.unwrap_or_else(|| {
        let ext = match format {
            CompressionFormat::Gzip => "gz",
            CompressionFormat::Zstd => "zst",
        };
        PathBuf::from(format!("{}.{}", input.display(), ext))
    });

    let input_file =
        File::open(input).with_context(|| format!("Failed to open input: {}", input.display()))?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create output: {}", output_path.display()))?;
    let writer = BufWriter::new(output_file);

    match format {
        CompressionFormat::Gzip => {
            let level = level.min(9);
            let mut encoder = GzEncoder::new(writer, Compression::new(level));
            std::io::copy(&mut reader, &mut encoder)?;
            encoder.finish()?;
        }
        CompressionFormat::Zstd => {
            // ruzstd is decode-only, so we use flate2 for zstd-like compression
            // In a real implementation, you'd use the `zstd` crate with vendored feature
            anyhow::bail!("Zstd compression requires the 'zstd' crate. Use gzip for now.");
        }
    }

    let input_size = std::fs::metadata(input)?.len();
    let output_size = std::fs::metadata(&output_path)?.len();
    let ratio = if input_size > 0 {
        (output_size as f64 / input_size as f64) * 100.0
    } else {
        100.0
    };

    println!(
        "{} {} -> {} ({:.1}%)",
        "Compressed:".green(),
        input.display(),
        output_path.display(),
        ratio
    );
    Ok(())
}

fn cmd_decompress(input: &PathBuf, out_file: Option<PathBuf>) -> Result<()> {
    let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");

    let output_path = out_file.unwrap_or_else(|| {
        let stem = input.file_stem().unwrap_or_default();
        input.with_file_name(stem)
    });

    let input_file =
        File::open(input).with_context(|| format!("Failed to open input: {}", input.display()))?;
    let reader = BufReader::new(input_file);

    let output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create output: {}", output_path.display()))?;
    let mut writer = BufWriter::new(output_file);

    match ext {
        "gz" | "gzip" => {
            let mut decoder = GzDecoder::new(reader);
            std::io::copy(&mut decoder, &mut writer)?;
        }
        "zst" | "zstd" => {
            let mut decoder = ruzstd::streaming_decoder::StreamingDecoder::new(reader)
                .map_err(|e| anyhow::anyhow!("Failed to create zstd decoder: {}", e))?;
            let mut buffer = Vec::new();
            decoder
                .read_to_end(&mut buffer)
                .context("Failed to decompress zstd")?;
            writer.write_all(&buffer)?;
        }
        _ => {
            anyhow::bail!("Unknown compression format. Supported: .gz, .gzip, .zst, .zstd");
        }
    }

    writer.flush()?;

    println!(
        "{} {} -> {}",
        "Decompressed:".green(),
        input.display(),
        output_path.display()
    );
    Ok(())
}
