//! Cryptocurrency and blockchain data generation.
//!
//! Generates realistic cryptocurrency addresses, transaction hashes,
//! wallet data, and blockchain-related identifiers.
//!
//! # Examples
//!
//! ```
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use dx_datagen::crypto;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Generate addresses
//! let btc = crypto::bitcoin_address(&mut rng);
//! let eth = crypto::ethereum_address(&mut rng);
//!
//! // Generate transaction data
//! let tx_hash = crypto::transaction_hash(&mut rng);
//! let block = crypto::block_number(&mut rng);
//! ```

use rand::Rng;

// Constants for address generation
const HEX_CHARS: &[u8] = b"0123456789abcdef";
const BASE58_CHARS: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BECH32_CHARS: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

/// Generate a random hexadecimal string of specified length.
fn random_hex<R: Rng + ?Sized>(rng: &mut R, len: usize) -> String {
    (0..len)
        .map(|_| HEX_CHARS[rng.random_range(0..16)] as char)
        .collect()
}

/// Generate a random Base58 string of specified length.
fn random_base58<R: Rng + ?Sized>(rng: &mut R, len: usize) -> String {
    (0..len)
        .map(|_| BASE58_CHARS[rng.random_range(0..58)] as char)
        .collect()
}

/// Generate a random Bech32 string of specified length.
fn random_bech32<R: Rng + ?Sized>(rng: &mut R, len: usize) -> String {
    (0..len)
        .map(|_| BECH32_CHARS[rng.random_range(0..32)] as char)
        .collect()
}

// =============================================================================
// Bitcoin
// =============================================================================

/// Generate a legacy Bitcoin address (P2PKH, starts with 1).
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::crypto::bitcoin_address;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let addr = bitcoin_address(&mut rng);
/// assert!(addr.starts_with('1'));
/// assert!(addr.len() >= 26 && addr.len() <= 35);
/// ```
pub fn bitcoin_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    let len = rng.random_range(26..=34);
    format!("1{}", random_base58(rng, len - 1))
}

/// Generate a Bitcoin P2SH address (starts with 3).
pub fn bitcoin_p2sh<R: Rng + ?Sized>(rng: &mut R) -> String {
    let len = rng.random_range(26..=34);
    format!("3{}", random_base58(rng, len - 1))
}

/// Generate a Bitcoin SegWit (Bech32) address (starts with bc1).
pub fn bitcoin_segwit<R: Rng + ?Sized>(rng: &mut R) -> String {
    // bc1q for P2WPKH (42 chars total), bc1p for Taproot (62 chars)
    if rng.random_bool(0.7) {
        // P2WPKH - more common
        format!("bc1q{}", random_bech32(rng, 38))
    } else {
        // Taproot (P2TR)
        format!("bc1p{}", random_bech32(rng, 58))
    }
}

/// Generate a Bitcoin testnet address.
pub fn bitcoin_testnet<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefixes = ["m", "n", "2", "tb1q"];
    let prefix = prefixes[rng.random_range(0..prefixes.len())];

    if prefix.starts_with("tb1") {
        format!("{}{}", prefix, random_bech32(rng, 38))
    } else {
        let len = rng.random_range(26..=34);
        format!("{}{}", prefix, random_base58(rng, len - 1))
    }
}

// =============================================================================
// Ethereum
// =============================================================================

/// Generate an Ethereum address (0x + 40 hex chars).
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::crypto::ethereum_address;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let addr = ethereum_address(&mut rng);
/// assert!(addr.starts_with("0x"));
/// assert_eq!(addr.len(), 42);
/// ```
pub fn ethereum_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("0x{}", random_hex(rng, 40))
}

/// Generate an Ethereum address with checksum (mixed case).
pub fn ethereum_address_checksum<R: Rng + ?Sized>(rng: &mut R) -> String {
    let hex: String = (0..40)
        .map(|_| {
            let c = HEX_CHARS[rng.random_range(0..16)] as char;
            if c.is_ascii_alphabetic() && rng.random_bool(0.5) {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect();
    format!("0x{}", hex)
}

// =============================================================================
// Other Cryptocurrencies
// =============================================================================

/// Generate a Litecoin address (L or M prefix for mainnet).
pub fn litecoin_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = if rng.random_bool(0.5) { "L" } else { "M" };
    let len = rng.random_range(26..=34);
    format!("{}{}", prefix, random_base58(rng, len - 1))
}

/// Generate a Dogecoin address (D prefix).
pub fn dogecoin_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    let len = rng.random_range(26..=34);
    format!("D{}", random_base58(rng, len - 1))
}

/// Generate a Solana address (Base58, 32-44 chars).
pub fn solana_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    random_base58(rng, 44)
}

/// Generate a Cardano (ADA) Shelley address.
pub fn cardano_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("addr1{}", random_bech32(rng, 98))
}

/// Generate a Polkadot address.
pub fn polkadot_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("1{}", random_base58(rng, 47))
}

/// Generate a Cosmos address.
pub fn cosmos_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("cosmos1{}", random_bech32(rng, 38))
}

/// Generate a TRON address (starts with T).
pub fn tron_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("T{}", random_base58(rng, 33))
}

/// Generate an XRP (Ripple) address.
pub fn xrp_address<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("r{}", random_base58(rng, 33))
}

// =============================================================================
// Transaction & Block Data
// =============================================================================

/// Generate a transaction hash (64 hex chars).
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::crypto::transaction_hash;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let hash = transaction_hash(&mut rng);
/// assert_eq!(hash.len(), 64);
/// ```
pub fn transaction_hash<R: Rng + ?Sized>(rng: &mut R) -> String {
    random_hex(rng, 64)
}

/// Generate a transaction hash with 0x prefix (Ethereum style).
pub fn transaction_hash_0x<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("0x{}", random_hex(rng, 64))
}

/// Generate a block hash (64 hex chars).
pub fn block_hash<R: Rng + ?Sized>(rng: &mut R) -> String {
    random_hex(rng, 64)
}

/// Generate a block hash with 0x prefix.
pub fn block_hash_0x<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("0x{}", random_hex(rng, 64))
}

/// Generate a realistic block number.
pub fn block_number<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    rng.random_range(1..20_000_000)
}

/// Generate a block number for a specific chain.
pub fn block_number_for_chain<R: Rng + ?Sized>(rng: &mut R, chain: &str) -> u64 {
    match chain.to_lowercase().as_str() {
        "ethereum" | "eth" => rng.random_range(15_000_000..20_000_000),
        "bitcoin" | "btc" => rng.random_range(700_000..850_000),
        "polygon" | "matic" => rng.random_range(40_000_000..55_000_000),
        "arbitrum" => rng.random_range(100_000_000..200_000_000),
        "optimism" => rng.random_range(100_000_000..120_000_000),
        _ => rng.random_range(1..10_000_000),
    }
}

/// Generate a gas price in Gwei.
pub fn gas_price_gwei<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    rng.random_range(5..200)
}

/// Generate a gas limit.
pub fn gas_limit<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    let common_limits = [21000, 50000, 100000, 200000, 500000, 1000000];
    common_limits[rng.random_range(0..common_limits.len())]
}

/// Generate a nonce (transaction count).
pub fn nonce<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    rng.random_range(0..10000)
}

// =============================================================================
// Wallet & Keys
// =============================================================================

/// Generate a BIP39 mnemonic phrase (12 words).
///
/// Note: These are random words, not cryptographically valid mnemonics.
pub fn mnemonic_12<R: Rng + ?Sized>(rng: &mut R) -> String {
    let words = BIP39_WORDS;
    (0..12)
        .map(|_| words[rng.random_range(0..words.len())])
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a BIP39 mnemonic phrase (24 words).
pub fn mnemonic_24<R: Rng + ?Sized>(rng: &mut R) -> String {
    let words = BIP39_WORDS;
    (0..24)
        .map(|_| words[rng.random_range(0..words.len())])
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a private key (64 hex chars).
pub fn private_key<R: Rng + ?Sized>(rng: &mut R) -> String {
    random_hex(rng, 64)
}

/// Generate a private key with 0x prefix.
pub fn private_key_0x<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("0x{}", random_hex(rng, 64))
}

/// Generate a public key (128 hex chars, uncompressed).
pub fn public_key<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("04{}", random_hex(rng, 128))
}

/// Generate a compressed public key (66 hex chars).
pub fn public_key_compressed<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = if rng.random_bool(0.5) { "02" } else { "03" };
    format!("{}{}", prefix, random_hex(rng, 64))
}

// =============================================================================
// NFT & Token Data
// =============================================================================

/// Generate an NFT token ID.
pub fn nft_token_id<R: Rng + ?Sized>(rng: &mut R) -> String {
    rng.random_range(1..100000u64).to_string()
}

/// Generate an NFT token ID (large range).
pub fn nft_token_id_large<R: Rng + ?Sized>(rng: &mut R) -> String {
    rng.random_range(1..u64::MAX).to_string()
}

/// Generate an ERC-20 token symbol.
pub fn token_symbol<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const SYMBOLS: &[&str] = &[
        "USDT", "USDC", "DAI", "WETH", "WBTC", "LINK", "UNI", "AAVE", "CRV", "MKR", "SNX", "COMP",
        "YFI", "SUSHI", "BAL", "REN", "KNC", "ZRX", "LRC", "ENJ", "MANA", "SAND", "AXS", "GALA",
        "IMX", "APE", "SHIB", "PEPE", "FLOKI", "DOGE",
    ];
    SYMBOLS[rng.random_range(0..SYMBOLS.len())]
}

/// Generate a token name.
pub fn token_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const NAMES: &[&str] = &[
        "Tether USD",
        "USD Coin",
        "Dai Stablecoin",
        "Wrapped Ether",
        "Wrapped Bitcoin",
        "Chainlink",
        "Uniswap",
        "Aave",
        "Curve DAO Token",
        "Maker",
        "Synthetix",
        "Compound",
        "yearn.finance",
        "SushiSwap",
        "Balancer",
        "Ren",
        "Kyber Network",
        "0x Protocol",
        "Loopring",
        "Enjin Coin",
        "Decentraland",
        "The Sandbox",
        "Axie Infinity",
        "Gala",
        "Immutable X",
    ];
    NAMES[rng.random_range(0..NAMES.len())]
}

/// Generate a token amount (wei/smallest unit).
pub fn token_amount_wei<R: Rng + ?Sized>(rng: &mut R) -> String {
    let amount: u128 = rng.random_range(1_000_000_000_000_000..1_000_000_000_000_000_000_000);
    amount.to_string()
}

/// Generate a token amount (human readable).
pub fn token_amount<R: Rng + ?Sized>(rng: &mut R) -> String {
    let whole = rng.random_range(0..10000u64);
    let decimal = rng.random_range(0..1000000u64);
    format!("{}.{:06}", whole, decimal)
}

// =============================================================================
// DeFi Specific
// =============================================================================

/// Generate a liquidity pool address.
pub fn liquidity_pool<R: Rng + ?Sized>(rng: &mut R) -> String {
    ethereum_address(rng)
}

/// Generate a DEX name.
pub fn dex_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const DEXES: &[&str] = &[
        "Uniswap",
        "SushiSwap",
        "Curve",
        "Balancer",
        "1inch",
        "dYdX",
        "PancakeSwap",
        "QuickSwap",
        "TraderJoe",
        "SpookySwap",
        "Raydium",
        "Orca",
        "Jupiter",
    ];
    DEXES[rng.random_range(0..DEXES.len())]
}

/// Generate a blockchain network name.
pub fn network_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const NETWORKS: &[&str] = &[
        "Ethereum",
        "Polygon",
        "Arbitrum",
        "Optimism",
        "Base",
        "Avalanche",
        "BNB Chain",
        "Fantom",
        "Solana",
        "Cardano",
        "Polkadot",
        "Cosmos",
        "Near",
        "Aptos",
        "Sui",
        "zkSync Era",
        "Linea",
        "Scroll",
    ];
    NETWORKS[rng.random_range(0..NETWORKS.len())]
}

/// Generate a chain ID.
pub fn chain_id<R: Rng + ?Sized>(rng: &mut R) -> u64 {
    const CHAIN_IDS: &[u64] = &[
        1,     // Ethereum
        137,   // Polygon
        42161, // Arbitrum
        10,    // Optimism
        8453,  // Base
        43114, // Avalanche
        56,    // BNB Chain
        250,   // Fantom
        324,   // zkSync Era
        59144, // Linea
    ];
    CHAIN_IDS[rng.random_range(0..CHAIN_IDS.len())]
}

/// Generate a smart contract address (same as Ethereum address).
pub fn smart_contract<R: Rng + ?Sized>(rng: &mut R) -> String {
    ethereum_address(rng)
}

// BIP39 word list (subset for realistic generation)
const BIP39_WORDS: &[&str] = &[
    "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd",
    "abuse", "access", "accident", "account", "accuse", "achieve", "acid", "acoustic", "acquire",
    "across", "act", "action", "actor", "actress", "actual", "adapt", "add", "addict", "address",
    "adjust", "admit", "adult", "advance", "advice", "aerobic", "affair", "afford", "afraid",
    "again", "age", "agent", "agree", "ahead", "aim", "air", "airport", "aisle", "alarm", "album",
    "alcohol", "alert", "alien", "all", "alley", "allow", "almost", "alone", "alpha", "already",
    "also", "alter", "always", "amateur", "amazing", "among", "amount", "amused", "analyst",
    "anchor", "ancient", "anger", "angle", "angry", "animal", "ankle", "announce", "annual",
    "another", "answer", "antenna", "antique", "anxiety", "any", "apart", "apology", "appear",
    "apple", "approve", "april", "arch", "arctic", "area", "arena", "argue", "arm", "armed",
    "armor", "army", "around", "arrange", "arrest", "arrive", "arrow", "art", "artefact", "artist",
    "artwork", "ask", "aspect", "assault", "asset", "assist", "assume", "asthma", "athlete",
    "atom", "attack", "attend", "attitude", "attract", "auction", "audit", "august", "aunt",
    "author", "auto", "autumn", "average", "avocado", "avoid", "awake", "aware", "away", "awesome",
    "awful", "awkward", "axis", "baby", "bachelor", "bacon", "badge", "bag", "balance", "balcony",
    "ball", "bamboo", "banana", "banner", "bar", "barely", "bargain", "barrel", "base", "basic",
    "basket", "battle", "beach", "bean", "beauty", "because", "become", "beef", "before", "begin",
    "behave", "behind", "believe", "below", "belt", "bench", "benefit", "best", "betray", "better",
    "between", "beyond", "bicycle", "bid", "bike", "bind", "biology", "bird", "birth", "bitter",
    "black", "blade", "blame", "blanket", "blast", "bleak", "bless", "blind", "blood", "blossom",
    "blouse", "blue", "blur", "blush", "board", "boat", "body", "boil", "bomb", "bone", "bonus",
    "book", "boost", "border", "boring", "borrow", "boss", "bottom", "bounce", "box", "boy",
    "bracket", "brain", "brand", "brass", "brave", "bread", "breeze", "brick", "bridge", "brief",
    "bright", "bring", "brisk", "broccoli", "broken", "bronze", "broom", "brother", "brown",
    "brush", "bubble", "buddy", "budget", "buffalo", "build", "bulb", "bulk", "bullet", "bundle",
    "bunker", "burden", "burger", "burst", "bus", "business", "busy", "butter", "buyer", "buzz",
    "cabbage", "cabin", "cable", "cactus", "cage", "cake", "call", "calm", "camera", "camp", "can",
];

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    #[test]
    fn test_bitcoin_address() {
        let mut rng = test_rng();
        let addr = bitcoin_address(&mut rng);
        assert!(addr.starts_with('1'));
        assert!(addr.len() >= 26 && addr.len() <= 35);
    }

    #[test]
    fn test_bitcoin_segwit() {
        let mut rng = test_rng();
        let addr = bitcoin_segwit(&mut rng);
        assert!(addr.starts_with("bc1"));
    }

    #[test]
    fn test_ethereum_address() {
        let mut rng = test_rng();
        let addr = ethereum_address(&mut rng);
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
    }

    #[test]
    fn test_transaction_hash() {
        let mut rng = test_rng();
        let hash = transaction_hash(&mut rng);
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_mnemonic_12() {
        let mut rng = test_rng();
        let mnemonic = mnemonic_12(&mut rng);
        assert_eq!(mnemonic.split_whitespace().count(), 12);
    }

    #[test]
    fn test_mnemonic_24() {
        let mut rng = test_rng();
        let mnemonic = mnemonic_24(&mut rng);
        assert_eq!(mnemonic.split_whitespace().count(), 24);
    }

    #[test]
    fn test_private_key() {
        let mut rng = test_rng();
        let key = private_key(&mut rng);
        assert_eq!(key.len(), 64);
    }

    #[test]
    fn test_solana_address() {
        let mut rng = test_rng();
        let addr = solana_address(&mut rng);
        assert_eq!(addr.len(), 44);
    }

    #[test]
    fn test_token_symbol() {
        let mut rng = test_rng();
        let symbol = token_symbol(&mut rng);
        assert!(!symbol.is_empty());
    }

    #[test]
    fn test_network_name() {
        let mut rng = test_rng();
        let network = network_name(&mut rng);
        assert!(!network.is_empty());
    }
}
