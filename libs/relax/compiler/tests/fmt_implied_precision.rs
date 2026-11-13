// `.` with no count is a legal precision: rustc parses it as `Count::Implied`, which
// `FormatSpec::precision()` reports as *no* precision (rustc_parse `parse_spec`/`parse_count`), so
// `{:.e}`, `{:.}`, `{:.?}` all format exactly as the spec without a precision does, and `{:.2e}`
// keeps an explicit count. polars-io 0.55.2 writes `write!(buf, "{item:.e}")`, which used to make the
// crate unloadable.
#[test]
fn implied_precision_is_no_precision() {
    let item = 1.5f64;
    assert_eq!(format!("{item:.e}"), "1.5e0");
    assert_eq!(format!("{item:.}"), "1.5");
    assert_eq!(format!("{item:.?}"), "1.5");
    assert_eq!(format!("{item:.2e}"), "1.50e0");
}

// `.*` and `.*$` still take their precision from an argument.
#[test]
fn star_precision_still_takes_an_argument() {
    let item = 1.5f64;
    assert_eq!(format!("{item:.*}", 2), "1.50");
    assert_eq!(format!("{0:.*}", 3, item), "1.500");
}

// A width with an implied precision, and a named argument's implied precision.
#[test]
fn implied_precision_beside_width_and_names() {
    let v = 12.5f64;
    assert_eq!(format!("{v:>8.}"), "    12.5");
    assert_eq!(format!("{v:.}", v = v), "12.5");
}
