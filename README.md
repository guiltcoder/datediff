# Datediff
A Rust Library for getting difference of two dates as Interval i.e days, months and years.


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
datediff = "0.1"
```

## Quick Start

```
use chrono::NaiveDate;

use datediff::get_diff;

let start_date = NaiveDate::from_ymd(1947, 8, 15);
let end_date = NaiveDate::from_ymd(1950, 1, 26);

println!("Duration is {}", get_diff(&start_date, &end_date));
```