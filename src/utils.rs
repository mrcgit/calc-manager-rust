use chrono::{Duration,DateTime,Utc,FixedOffset,TimeZone,Datelike};
use rust_decimal::prelude::*;
use serde::Deserialize;


pub fn compute(active_client_predictions: &[CalcManagerPrediction], bonus_configurations: &BcsBonus) -> f64 {
    let total_odd_no_bonus = total_odd_without_bonus(active_client_predictions);
    let bonus_percentage = compute_bonus(active_client_predictions, bonus_configurations);
    truncate_number_to(bonus_percentage * total_odd_no_bonus, 6)
}

fn compute_bonus(predictions: &[CalcManagerPrediction], config_ticket: &BcsBonus) -> f64 {
    if config_ticket.bonus_expiration_days <= 0 {
        return 1.0;
    }

    let valid_predictions: Vec<_> = predictions
        .iter()
        .filter(|&prediction| prediction.odd > config_ticket.bonus_min_odd)
        .collect();

    if valid_predictions.len() < config_ticket.bonus_min_num_outcomes as usize {
        return 1.0;
    }

    let date_to_compare = get_date_after_days(config_ticket.bonus_expiration_days);
    let num_outcomes = valid_predictions.len() as i64 - config_ticket.bonus_min_num_outcomes as i64 + 1;

    if let Some(_first_not_valid_for_date) =
        get_first_prediction_not_valid_for_date(predictions, date_to_compare)
    {
        return 1.0;
    }

    let bonus_factor = (config_ticket.bonus_percentage / 100.0).powi(num_outcomes as i32);
    bonus_factor
}

fn get_first_prediction_not_valid_for_date(
    predictions: &[CalcManagerPrediction],
    bonus_expiration_date: DateTime<FixedOffset>,
) -> Option<&CalcManagerPrediction> {
    predictions
        .iter()
        .find(|&prediction| !is_prediction_date_valid_for_bonus(
            &prediction.event_timestamp,
            bonus_expiration_date,
        ))
}

fn is_prediction_date_valid_for_bonus(
    regulator_date: &str,
    bonus_expiration_date: DateTime<FixedOffset>,
) -> bool {
    let date_clean = DateTime::parse_from_rfc3339(regulator_date).unwrap();
    date_clean <= bonus_expiration_date
}

fn total_odd_without_bonus(predictions: &[CalcManagerPrediction]) -> f64 {
    predictions.iter().fold(1.0, |total, prediction| {
        let odd = Decimal::from_f64(prediction.odd as f64).unwrap();
        let cent =  Decimal::from_f64(0.01_f64).unwrap();
        let tot = Decimal::from_f64(total).unwrap();
        (tot * odd * cent).to_f64().unwrap()
    })
}



pub fn get_date_after_days(num_days: i64)->DateTime<FixedOffset>  {
    let now: DateTime<Utc> = Utc::now() + Duration::days(num_days);
    let first = Utc.with_ymd_and_hms(now.year(),now.month(), now.day(), 23,59,59).unwrap();

    let formatted_date_time = first.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    DateTime::parse_from_rfc3339(&formatted_date_time).unwrap()
}


pub fn truncate_number_to(value:f64, decimals: usize)->f64{
    let multiplier = 10_f64.powi(decimals as i32);
    let truncated_value = (value * multiplier).trunc()/multiplier;
    truncated_value
}

#[derive(Deserialize)]
pub struct CalcManagerPrediction {
    pub event_timestamp: String,
    pub odd: i32,
    pub status: i32,
}
#[derive(Deserialize)]
pub struct BcsBonus {
    pub cardinality: i32,
    pub bonus_percentage: f64,
    pub bonus_min_num_outcomes: i32,
    pub bonus_min_odd: i32,
    pub bonus_expiration_days: i64,
}



