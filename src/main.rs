mod saints;

use chrono::Local;

fn main() {
    // Get the date and time
    let current_date = Local::now();
    let parsed_month = current_date.format("%B").to_string().to_lowercase();
    let parsed_date = current_date.format("%d").to_string();

    let months = saints::months();

    if let Some(month) = months.get(&parsed_month) {
        if let Some(saint) = month.get(&parsed_date) {
            println!("\nSaint of the Day:\n{}", saint);            
        } else {
            println!("\nNo saint of the day. Go pray your rosary");
        }
    }
}
