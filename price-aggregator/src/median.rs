dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

/// Returns the sorted middle, or the average of the two middle indexed items if the
/// vector has an even number of elements.
pub fn calculate<BigUint: BigUintApi>(mut list: Vec<BigUint>) -> Result<Option<BigUint>, SCError>
where
    BigUint: BigUintApi,
{
    if list.is_empty() {
        return Result::Ok(None);
    }
    list.sort();
    let len = list.len();
    let middle_index = len / 2;
    if len % 2 == 0 {
        let median1 = list.get(middle_index - 1).ok_or("median1 invalid index")?;
        let median2 = list.get(middle_index).ok_or("median2 invalid index")?;
        Result::Ok(Some((median1.clone() + median2.clone()) / 2u64.into()))
    } else {
        let median = list.get(middle_index).ok_or("median invalid index")?;
        Result::Ok(Some(median.clone()))
    }
}
