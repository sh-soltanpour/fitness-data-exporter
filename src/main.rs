mod read_csv;

use diesel::{PgConnection, RunQueryDsl};
use chrono::NaiveDate;

use fitness_data_exporter::{establish_connection, schema};
use fitness_data_exporter::models::{Activity, NewActivity};

fn main() {
    let connection = establish_connection();
    let records = read_csv::parse("src/dailyActivity_merged.csv")
        .expect("Parse csv file");

    for record in records {
        let date = NaiveDate::parse_from_str(
            &record.ActivityDate.to_string(), "%m/%d/%Y")
            .expect("Parse date");

        create_daily_activity(&connection,
                              &NewActivity {
                                  user_id: &record.Id.to_string(),
                                  activity_date: &date,
                                  total_steps: &record.TotalSteps,
                                  total_distance: &record.TotalDistance,
                                  calories: &record.Calories,
                              },
        );
    }
}

pub fn create_daily_activity(
    conn: &PgConnection,
    new_activity: &NewActivity,
) -> Activity {
    diesel::insert_into(schema::daily_activity::table)
        .values(new_activity)
        .get_result(conn)
        .expect("Unable to create auction")
}
