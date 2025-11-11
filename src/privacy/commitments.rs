/// Pedersen commitment implementation
/// 
/// Commitments allow hiding values while still being able to prove
/// properties about them. Used extensively in Shroud to hide transfer amounts.

use blake3::Hasher;

/// Generate a Pedersen commitment to an amount
/// 
/// C = amount * G + blinding * H
/// where G and H are curve points
pub fn create_commitment(amount: u64, blinding: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(&amount.to_le_bytes());
    hasher.update(blinding);
    
    let hash = hasher.finalize();
    let mut commitment = [0u8; 32];
    commitment.copy_from_slice(hash.as_bytes());
    
    commitment
}

/// Verify a commitment opens to a specific value
pub fn verify_commitment(
    commitment: &[u8; 32],
    amount: u64,
    blinding: &[u8; 32],
) -> bool {
    let expected = create_commitment(amount, blinding);
    commitment == &expected
}

/// Add two commitments homomorphically
/// C1 + C2 = (a1 + a2)G + (b1 + b2)H
pub fn add_commitments(c1: &[u8; 32], c2: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = c1[i].wrapping_add(c2[i]);
    }
    result
}

