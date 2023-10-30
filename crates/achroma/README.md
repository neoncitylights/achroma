# achroma

[![License](https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue)](#license)
[![CI](https://github.com/neoncitylights/rust/actions/workflows/main.yml/badge.svg)](https://github.com/neoncitylights/rust/actions/workflows/main.yml)
[![Security audit](https://github.com/neoncitylights/rust/actions/workflows/security-audit.yml/badge.svg)](https://github.com/neoncitylights/rust/actions/workflows/security-audit.yml)
[![codecov](https://codecov.io/gh/neoncitylights/rust/branch/main/graph/badge.svg?token=6ZSIWAQTHU)](https://codecov.io/gh/neoncitylights/rust)

A tiny crate for encoding data related to color vision and color vision deficiency (CVD).

## Install

```shell
cargo add achroma
```

## Usage


```rs
use achroma::*;

// Partial blindness to red light
let protanomaly = ColorVision::Protanomaly;
let protanomaly_summary = ConeCellSummary::from(protanomaly);

// query for information of overall CVD
assert_eq!(protanomaly.is_red_green(), true);
assert_eq!(protanomaly.is_anomalous_trichromacy(), true);

// query for individual cone cells by length
assert_eq!(protanomaly_summary.long(),   ConeCellCond::Anomalous);
assert_eq!(protanomaly_summary.medium(), ConeCellCond::Normal);
assert_eq!(protanomaly_summary.short(),  ConeCellCond::Normal);
assert_eq!(protanomaly_summary.is_cone_normal(ConeCell::Long), false);

// query for individual cone cells by color
assert_eq!(protanomaly_summary.red().is_anomalous(), true);
assert_eq!(protanomaly_summary.green(), ConeCellCond::Normal);
assert_eq!(protanomaly_summary['b'].is_normal(), true);
assert_eq!(protanomaly_summary['B'].is_normal(), true);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
