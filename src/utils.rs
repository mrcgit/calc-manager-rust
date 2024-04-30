use chrono::{Duration,DateTime,Utc,FixedOffset,TimeZone,Datelike};



pub fn get_date_after_days(num_days: i64)->DateTime<FixedOffset>  {
    let mut now: DateTime<Utc> = Utc::now() + Duration::days(num_days);
    let first = Utc.with_ymd_and_hms(now.year(),now.month(), now.day(), 23,59,59).unwrap();

    let formatted_date_time = first.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    DateTime::parse_from_rfc3339(&formatted_date_time).unwrap()
}


pub fn to_fixed_trunc(value:f64, decimals: usize)->f64{
    let multiplier = 10_f64.powi(decimals as i32);
    let truncated_value = (value * multiplier).trunc()/multiplier;
    truncated_value
}
#[derive(Debug)]
pub struct CalcManagerPrediction{
  pub  event_timestamp: String,
  pub  odd: f64,
  pub  status: i32

}

#[derive(Debug)]
pub struct BcsBonus{
    cardinality: u32,
    bonus_percentage: f64,
    bonus_min_num_outcomes: u32,
    bonus_min_odd: f64,
    bonus_expiration_days: u32
}



