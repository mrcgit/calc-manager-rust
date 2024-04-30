mod utils;
use chrono::{DateTime,Utc};
use utils::{get_date_after_days,to_fixed_trunc,CalcManagerPrediction};

fn main() {
    
    let date1 = get_date_after_days(064);
    let date_str = "2024-04-30T09:00:00Z";
    let date_time = DateTime::parse_from_rfc3339(date_str).unwrap();

    if date1 < date_time{
        println!("La data {} è minore di {}", date1,date_time);
    } else if date1 > date_time {
        println!("La data {} è maggiore di {}", date1,date_time);
    } else {
        println!("Le date sono uguali");
    }

    let num = to_fixed_trunc(1.23456789,6);

    let prediction = CalcManagerPrediction {
        event_timestamp: "2024-04-30T10:54:00".to_string(),
        odd: 2.5,
        status: 1
    };

    println!("{}",num);
    println!("{:.?}",prediction);

}
