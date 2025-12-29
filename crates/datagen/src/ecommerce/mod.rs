//! E-commerce data generation.
//!
//! Generates realistic e-commerce data including orders, shipping,
//! reviews, coupons, and customer data.
//!
//! # Examples
//!
//! ```
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use dx_datagen::ecommerce;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! let order_id = ecommerce::order_id(&mut rng);
//! let status = ecommerce::order_status(&mut rng);
//! let tracking = ecommerce::tracking_number(&mut rng);
//! ```

use rand::Rng;

// =============================================================================
// Order Data
// =============================================================================

/// Generate an order ID.
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::ecommerce::order_id;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let id = order_id(&mut rng);
/// assert!(id.starts_with("ORD-"));
/// ```
pub fn order_id<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("ORD-{:08}", rng.random_range(10000000..99999999u32))
}

/// Generate an order ID with custom prefix.
pub fn order_id_with_prefix<R: Rng + ?Sized>(rng: &mut R, prefix: &str) -> String {
    format!("{}-{:08}", prefix, rng.random_range(10000000..99999999u32))
}

/// Generate an order status.
pub fn order_status<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const STATUSES: &[&str] = &[
        "pending",
        "processing",
        "confirmed",
        "shipped",
        "out_for_delivery",
        "delivered",
        "cancelled",
        "refunded",
        "on_hold",
        "backordered",
    ];
    STATUSES[rng.random_range(0..STATUSES.len())]
}

/// Generate a payment status.
pub fn payment_status<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const STATUSES: &[&str] = &[
        "pending",
        "authorized",
        "captured",
        "completed",
        "failed",
        "refunded",
        "partially_refunded",
        "voided",
        "expired",
    ];
    STATUSES[rng.random_range(0..STATUSES.len())]
}

/// Generate a payment method.
pub fn payment_method<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const METHODS: &[&str] = &[
        "credit_card",
        "debit_card",
        "paypal",
        "apple_pay",
        "google_pay",
        "bank_transfer",
        "cash_on_delivery",
        "buy_now_pay_later",
        "crypto",
        "gift_card",
        "store_credit",
        "affirm",
        "klarna",
        "afterpay",
    ];
    METHODS[rng.random_range(0..METHODS.len())]
}

/// Generate an invoice number.
pub fn invoice_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!(
        "INV-{}-{:06}",
        2024 + rng.random_range(0..2i32),
        rng.random_range(1..999999u32)
    )
}

/// Generate a PO (Purchase Order) number.
pub fn po_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("PO-{:010}", rng.random_range(1000000000..9999999999u64))
}

// =============================================================================
// Shipping
// =============================================================================

/// Generate a tracking number.
///
/// # Example
/// ```
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use dx_datagen::ecommerce::tracking_number;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let tracking = tracking_number(&mut rng);
/// assert!(!tracking.is_empty());
/// ```
pub fn tracking_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    let carrier = rng.random_range(0..4);
    match carrier {
        0 => tracking_number_ups(rng),
        1 => tracking_number_fedex(rng),
        2 => tracking_number_usps(rng),
        _ => tracking_number_dhl(rng),
    }
}

/// Generate a UPS tracking number (1Z format).
pub fn tracking_number_ups<R: Rng + ?Sized>(rng: &mut R) -> String {
    let chars: String = (0..6)
        .map(|_| {
            let c = rng.random_range(0..36);
            if c < 26 {
                (b'A' + c as u8) as char
            } else {
                char::from_digit((c - 26) as u32, 10).unwrap()
            }
        })
        .collect();
    let nums: String = (0..10)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();
    format!("1Z{}{}", chars, nums)
}

/// Generate a FedEx tracking number (12-22 digits).
pub fn tracking_number_fedex<R: Rng + ?Sized>(rng: &mut R) -> String {
    let len = rng.random_range(12..=15);
    (0..len)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect()
}

/// Generate a USPS tracking number (22 digits).
pub fn tracking_number_usps<R: Rng + ?Sized>(rng: &mut R) -> String {
    (0..22)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect()
}

/// Generate a DHL tracking number (10 digits).
pub fn tracking_number_dhl<R: Rng + ?Sized>(rng: &mut R) -> String {
    (0..10)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect()
}

/// Generate a shipping carrier name.
pub fn shipping_carrier<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const CARRIERS: &[&str] = &[
        "UPS",
        "FedEx",
        "USPS",
        "DHL",
        "Amazon Logistics",
        "OnTrac",
        "LaserShip",
        "Purolator",
        "Canada Post",
        "Royal Mail",
        "Deutsche Post",
        "Chronopost",
        "Poste Italiane",
        "Correos",
        "Australia Post",
    ];
    CARRIERS[rng.random_range(0..CARRIERS.len())]
}

/// Generate a shipping method.
pub fn shipping_method<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const METHODS: &[&str] = &[
        "Standard Shipping",
        "Express Shipping",
        "Overnight",
        "Two-Day",
        "Ground",
        "Priority",
        "Economy",
        "Same Day",
        "Free Shipping",
        "Freight",
        "White Glove",
        "Curbside Pickup",
        "In-Store Pickup",
    ];
    METHODS[rng.random_range(0..METHODS.len())]
}

/// Generate a shipping cost.
pub fn shipping_cost<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    let cost: f64 = rng.random_range(0.0..50.0);
    (cost * 100.0).round() / 100.0
}

/// Generate an estimated delivery date description.
pub fn delivery_estimate<R: Rng + ?Sized>(rng: &mut R) -> String {
    let days = rng.random_range(1..14);
    if days == 1 {
        "Tomorrow".to_string()
    } else if days <= 3 {
        format!("{} business days", days)
    } else {
        format!("{}-{} business days", days, days + 2)
    }
}

// =============================================================================
// Returns & Refunds
// =============================================================================

/// Generate a return reason.
pub fn return_reason<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const REASONS: &[&str] = &[
        "Wrong size",
        "Wrong color",
        "Defective product",
        "Not as described",
        "Changed mind",
        "Arrived too late",
        "Better price found elsewhere",
        "Received wrong item",
        "Missing parts",
        "Quality not as expected",
        "No longer needed",
        "Gift return",
        "Duplicate order",
        "Compatibility issue",
    ];
    REASONS[rng.random_range(0..REASONS.len())]
}

/// Generate a refund status.
pub fn refund_status<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const STATUSES: &[&str] = &[
        "requested",
        "approved",
        "processing",
        "completed",
        "rejected",
        "partial",
    ];
    STATUSES[rng.random_range(0..STATUSES.len())]
}

/// Generate a return merchandise authorization (RMA) number.
pub fn rma_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("RMA-{:08}", rng.random_range(10000000..99999999u32))
}

// =============================================================================
// Reviews & Ratings
// =============================================================================

/// Generate a product rating (1-5).
pub fn rating<R: Rng + ?Sized>(rng: &mut R) -> u8 {
    // Weighted toward higher ratings (more realistic)
    let weights = [5, 10, 15, 30, 40]; // 1-5 star weights
    let total: u32 = weights.iter().sum();
    let mut pick = rng.random_range(0..total);

    for (i, &weight) in weights.iter().enumerate() {
        if pick < weight {
            return (i + 1) as u8;
        }
        pick -= weight;
    }
    5
}

/// Generate a review title.
pub fn review_title<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const POSITIVE: &[&str] = &[
        "Great product!",
        "Highly recommend",
        "Exceeded expectations",
        "Love it!",
        "Perfect!",
        "Amazing quality",
        "Best purchase ever",
        "Would buy again",
        "Exactly what I needed",
        "Five stars!",
    ];
    const NEUTRAL: &[&str] = &[
        "It's okay",
        "Decent product",
        "As expected",
        "Average quality",
        "Does the job",
        "Nothing special",
        "Fair for the price",
    ];
    const NEGATIVE: &[&str] = &[
        "Disappointing",
        "Not worth it",
        "Poor quality",
        "Would not recommend",
        "Broke after a week",
        "Not as described",
        "Save your money",
    ];

    let sentiment = rng.random_range(0..10);
    if sentiment < 6 {
        POSITIVE[rng.random_range(0..POSITIVE.len())]
    } else if sentiment < 9 {
        NEUTRAL[rng.random_range(0..NEUTRAL.len())]
    } else {
        NEGATIVE[rng.random_range(0..NEGATIVE.len())]
    }
}

/// Generate a review snippet.
pub fn review_snippet<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const SNIPPETS: &[&str] = &[
        "This product exceeded all my expectations. Would definitely buy again!",
        "Great value for money. Shipping was fast and packaging was excellent.",
        "Exactly as described. Very happy with my purchase.",
        "Good quality but shipping took longer than expected.",
        "The product is okay but not sure if it's worth the price.",
        "Had some issues initially but customer service was helpful.",
        "Bought this as a gift and the recipient loved it!",
        "Works perfectly. Easy to set up and use.",
        "Nice design and feels premium. Highly recommended!",
        "Solid product overall. Minor imperfections but nothing major.",
    ];
    SNIPPETS[rng.random_range(0..SNIPPETS.len())]
}

/// Generate whether a review is verified purchase.
pub fn verified_purchase<R: Rng + ?Sized>(rng: &mut R) -> bool {
    rng.random_bool(0.85) // 85% are verified
}

/// Generate number of helpful votes.
pub fn helpful_votes<R: Rng + ?Sized>(rng: &mut R) -> u32 {
    if rng.random_bool(0.7) {
        rng.random_range(0..10)
    } else {
        rng.random_range(10..500)
    }
}

// =============================================================================
// Coupons & Discounts
// =============================================================================

/// Generate a coupon code.
pub fn coupon_code<R: Rng + ?Sized>(rng: &mut R) -> String {
    const PREFIXES: &[&str] = &[
        "SAVE", "DEAL", "PROMO", "DISCOUNT", "SPECIAL", "VIP", "FLASH", "SUMMER", "WINTER",
        "SPRING", "FALL", "HOLIDAY", "WELCOME", "THANKS",
    ];
    let prefix = PREFIXES[rng.random_range(0..PREFIXES.len())];
    let number = rng.random_range(5..50) * 5; // 5, 10, 15, ... 50
    format!("{}{}", prefix, number)
}

/// Generate a coupon code with custom format.
pub fn coupon_code_alphanumeric<R: Rng + ?Sized>(rng: &mut R) -> String {
    let chars: String = (0..8)
        .map(|_| {
            let c = rng.random_range(0..36);
            if c < 26 {
                (b'A' + c as u8) as char
            } else {
                char::from_digit((c - 26) as u32, 10).unwrap()
            }
        })
        .collect();
    chars
}

/// Generate a discount percentage.
pub fn discount_percent<R: Rng + ?Sized>(rng: &mut R) -> u8 {
    let common = [5, 10, 15, 20, 25, 30, 40, 50];
    common[rng.random_range(0..common.len())]
}

/// Generate a discount type.
pub fn discount_type<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const TYPES: &[&str] = &[
        "percentage",
        "fixed_amount",
        "free_shipping",
        "buy_one_get_one",
        "bundle",
        "loyalty_points",
        "first_order",
        "referral",
    ];
    TYPES[rng.random_range(0..TYPES.len())]
}

// =============================================================================
// Cart & Checkout
// =============================================================================

/// Generate a cart ID.
pub fn cart_id<R: Rng + ?Sized>(rng: &mut R) -> String {
    let hex: String = (0..32)
        .map(|_| {
            let c = rng.random_range(0..16);
            if c < 10 {
                char::from_digit(c, 10).unwrap()
            } else {
                (b'a' + (c - 10) as u8) as char
            }
        })
        .collect();
    hex
}

/// Generate a session ID.
pub fn session_id<R: Rng + ?Sized>(rng: &mut R) -> String {
    let chars: String = (0..64)
        .map(|_| {
            let c = rng.random_range(0..62);
            if c < 26 {
                (b'a' + c as u8) as char
            } else if c < 52 {
                (b'A' + (c - 26) as u8) as char
            } else {
                char::from_digit((c - 52) as u32, 10).unwrap()
            }
        })
        .collect();
    chars
}

/// Generate a checkout step.
pub fn checkout_step<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const STEPS: &[&str] = &[
        "cart",
        "shipping_address",
        "shipping_method",
        "payment",
        "review",
        "confirmation",
        "complete",
    ];
    STEPS[rng.random_range(0..STEPS.len())]
}

// =============================================================================
// SKU & Inventory
// =============================================================================

/// Generate a SKU (Stock Keeping Unit).
pub fn sku<R: Rng + ?Sized>(rng: &mut R) -> String {
    let letters: String = (0..3)
        .map(|_| (b'A' + rng.random_range(0..26)) as char)
        .collect();
    let numbers: String = (0..6)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();
    format!("{}-{}", letters, numbers)
}

/// Generate a barcode (EAN-13).
pub fn barcode_ean13<R: Rng + ?Sized>(rng: &mut R) -> String {
    let digits: String = (0..12)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();
    // Calculate check digit
    let mut sum = 0;
    for (i, c) in digits.chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            sum += d;
        } else {
            sum += d * 3;
        }
    }
    let check = (10 - (sum % 10)) % 10;
    format!("{}{}", digits, check)
}

/// Generate a barcode (UPC-A).
pub fn barcode_upc<R: Rng + ?Sized>(rng: &mut R) -> String {
    let digits: String = (0..11)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();
    // Calculate check digit
    let mut odd_sum = 0;
    let mut even_sum = 0;
    for (i, c) in digits.chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            odd_sum += d;
        } else {
            even_sum += d;
        }
    }
    let total = odd_sum * 3 + even_sum;
    let check = (10 - (total % 10)) % 10;
    format!("{}{}", digits, check)
}

/// Generate an inventory quantity.
pub fn inventory_quantity<R: Rng + ?Sized>(rng: &mut R) -> u32 {
    if rng.random_bool(0.1) {
        0 // Out of stock
    } else if rng.random_bool(0.3) {
        rng.random_range(1..10) // Low stock
    } else {
        rng.random_range(10..500) // Normal stock
    }
}

/// Generate a stock status.
pub fn stock_status<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    const STATUSES: &[&str] = &[
        "in_stock",
        "low_stock",
        "out_of_stock",
        "backordered",
        "preorder",
        "discontinued",
        "coming_soon",
    ];
    STATUSES[rng.random_range(0..STATUSES.len())]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    #[test]
    fn test_order_id() {
        let mut rng = test_rng();
        let id = order_id(&mut rng);
        assert!(id.starts_with("ORD-"));
        assert_eq!(id.len(), 12);
    }

    #[test]
    fn test_tracking_number() {
        let mut rng = test_rng();
        let tracking = tracking_number(&mut rng);
        assert!(!tracking.is_empty());
    }

    #[test]
    fn test_tracking_number_ups() {
        let mut rng = test_rng();
        let tracking = tracking_number_ups(&mut rng);
        assert!(tracking.starts_with("1Z"));
    }

    #[test]
    fn test_rating() {
        let mut rng = test_rng();
        for _ in 0..100 {
            let r = rating(&mut rng);
            assert!(r >= 1 && r <= 5);
        }
    }

    #[test]
    fn test_coupon_code() {
        let mut rng = test_rng();
        let code = coupon_code(&mut rng);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_sku() {
        let mut rng = test_rng();
        let sku = sku(&mut rng);
        assert!(sku.contains('-'));
        assert_eq!(sku.len(), 10);
    }

    #[test]
    fn test_barcode_ean13() {
        let mut rng = test_rng();
        let barcode = barcode_ean13(&mut rng);
        assert_eq!(barcode.len(), 13);
    }

    #[test]
    fn test_barcode_upc() {
        let mut rng = test_rng();
        let barcode = barcode_upc(&mut rng);
        assert_eq!(barcode.len(), 12);
    }
}
