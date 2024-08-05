pub fn calculate_lp_amount(amount_a: u64, amount_b: u64, amount_c: u64, amount_d: u64, amount_e: u64) -> std::result::Result<u64,String> {
    let sum = amount_a
        .checked_add(amount_b)
        .ok_or("Overflow occurred during addition of amount_a and amount_b")?
        .checked_add(amount_c)
        .ok_or("Overflow occurred during addition of sum and amount_c")?
        .checked_add(amount_d)
        .ok_or("Overflow occurred during addition of sum and amount_d")?
        .checked_add(amount_e)
        .ok_or("Overflow occurred during addition of sum and amount_e")?;

    Ok(sum)

}