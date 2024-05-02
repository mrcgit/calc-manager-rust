mod utils;
use utils::{CalcManagerPrediction, BcsBonus};
use rust_decimal::prelude::*;

fn main() {
    
 
        let predictions: Vec<CalcManagerPrediction> = vec![
        CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 500,
            status: 1,
        }, CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 335,
            status: 1,
        },
         CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 475,
            status: 1,
        },
         CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 206,
            status: 1,
        },
         CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 300,
            status: 1,
        },
         CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 400,
            status: 1,
        },
          CalcManagerPrediction {
            event_timestamp: "2024-05-03T16:30:00Z".to_string(),
            odd: 299,
            status: 1,
        }
    ];

        let bcs_bonus = BcsBonus {
        bonus_expiration_days: 7,
        bonus_min_num_outcomes: 5,
        bonus_min_odd: 124,
        bonus_percentage: 104.0,
        cardinality: 0,
    };

       let result = utils::compute(&predictions, &bcs_bonus);

       println!("{}", result);

}
