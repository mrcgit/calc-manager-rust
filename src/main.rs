mod utils;
use utils::{CalcManagerPrediction, BcsBonus};

fn main() {
    
  

        let predictions: Vec<CalcManagerPrediction> = vec![
        CalcManagerPrediction {
            event_timestamp: "2024-04-17T16:30:00Z".to_string(),
            odd: 640,
            status: 1,
        }, CalcManagerPrediction {
            event_timestamp: "2024-04-17T16:30:00Z".to_string(),
            odd: 750,
            status: 1,
        },
         CalcManagerPrediction {
            event_timestamp: "2024-04-17T16:30:00Z".to_string(),
            odd: 380,
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
