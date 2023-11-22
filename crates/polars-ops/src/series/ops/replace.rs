use polars_core::prelude::{DataType, PlHashMap, Series};
use polars_error::PolarsResult;

pub fn replace(
    s: &Series,
    mapping: &PlHashMap<i64, i64>,
    default: &Series,
    return_dtype: DataType,
) -> PolarsResult<Series> {
    match mapping.keys().len() {
        0 => return Ok(default.clone()),
        1 => (), // dispatch to when/then
        _ => (),
    };

    let s = s.clone();
    Ok(s)
}
