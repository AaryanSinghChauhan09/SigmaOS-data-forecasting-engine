// Integration test for SigmaOS-data-forecasting-engine
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_data_forecasting_engine as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
