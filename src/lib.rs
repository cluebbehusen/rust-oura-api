use paste::paste;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use typed_builder::TypedBuilder;

const API_BASE_URL: &str = "https://api.ouraring.com/v2/usercollection";

#[derive(Deserialize, Debug)]
pub struct Sample {
    pub interval: f32,
    pub items: Vec<Option<f32>>,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct ActivityContributors {
    pub meet_daily_targets: Option<u8>,
    pub move_every_hour: Option<u8>,
    pub recovery_time: Option<u8>,
    pub stay_active: Option<u8>,
    pub training_frequency: Option<u8>,
    pub training_volume: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailyActivity {
    pub id: String,
    pub class_5_min: Option<String>,
    pub score: Option<u8>,
    pub active_calories: u32,
    pub average_met_minutes: f32,
    pub contributors: ActivityContributors,
    pub equivalent_walking_distance: u32,
    pub high_activity_met_minutes: u32,
    pub high_activity_time: u32,
    pub inactivity_alerts: u32,
    pub low_activity_met_minutes: u32,
    pub low_activity_time: u32,
    pub medium_activity_met_minutes: u32,
    pub medium_activity_time: u32,
    pub met: Sample,
    pub meters_to_target: u32,
    pub non_wear_time: u32,
    pub resting_time: u32,
    pub sedentary_met_minutes: u32,
    pub sedentary_time: u32,
    pub steps: u32,
    pub target_calories: u32,
    pub target_meters: u32,
    pub total_calories: u32,
    pub day: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct ReadinessContributors {
    pub activity_balance: Option<u8>,
    pub body_temperature: Option<u8>,
    pub hrv_balance: Option<u8>,
    pub previous_day_activity: Option<u8>,
    pub previous_night: Option<u8>,
    pub recovery_index: Option<u8>,
    pub resting_heart_rate: Option<u8>,
    pub sleep_balance: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailyReadiness {
    pub id: String,
    pub contributors: ReadinessContributors,
    pub day: String,
    pub score: Option<u8>,
    pub temperature_deviation: Option<f32>,
    pub temperature_trend_deviation: Option<f32>,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct SleepContributors {
    pub deep_sleep: Option<u8>,
    pub efficiency: Option<u8>,
    pub latency: Option<u8>,
    pub rem_sleep: Option<u8>,
    pub restfulness: Option<u8>,
    pub timing: Option<u8>,
    pub total_sleep: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailySleep {
    pub id: String,
    pub contributors: SleepContributors,
    pub day: String,
    pub timestamp: String,
    pub score: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct DailySpO2AggregatedValues {
    pub average: f32,
}

#[derive(Deserialize, Debug)]
pub struct DailySpO2 {
    pub id: String,
    pub day: String,
    pub spo2_percentage: Option<DailySpO2AggregatedValues>,
}

#[derive(Deserialize, Debug)]
pub struct HeartRate {
    pub bpm: u8,
    pub source: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct PersonalInfo {
    pub id: String,
    pub age: Option<u32>,
    pub weight: Option<f32>,
    pub height: Option<f32>,
    pub biological_sex: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RestModeEpisode {
    pub tags: Vec<String>,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct RestModePeriod {
    pub id: String,
    pub end_day: Option<String>,
    pub end_time: Option<String>,
    pub episodes: Vec<RestModeEpisode>,
    pub start_day: String,
    pub start_time: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RingColor {
    GlossyBlack,
    StealthBlack,
    Rose,
    Silver,
    GlossyGold,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RingDesign {
    Heritage,
    Horizon,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RingHardwareType {
    Gen1,
    Gen2,
    Gen2m,
    Gen3,
}

#[derive(Deserialize, Debug)]
pub struct RingConfiguration {
    pub id: String,
    pub color: Option<RingColor>,
    pub design: Option<RingDesign>,
    pub firmware_version: Option<String>,
    pub hardware_type: Option<RingHardwareType>,
    pub set_up_at: Option<String>,
    pub size: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MomentType {
    Breathing,
    Meditation,
    Nap,
    Relaxation,
    Rest,
    BodyStatus,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MomentMood {
    Bad,
    Worse,
    Same,
    Good,
    Great,
}

#[derive(Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub day: String,
    pub start_datetime: String,
    pub end_datetime: String,
    pub r#type: MomentType,
    pub heart_rate: Option<Sample>,
    pub heart_rate_variability: Option<Sample>,
    pub mood: Option<MomentMood>,
    pub motion_count: Option<Sample>,
}

#[derive(Deserialize, Debug)]
pub struct ReadinessSummary {
    pub contributors: ReadinessContributors,
    pub score: Option<u8>,
    pub temperature_devation: Option<f32>,
    pub temperature_trend_deviation: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SleepAlgorithmVersion {
    V1,
    V2,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SleepType {
    Deleted,
    Sleep,
    LongSleep,
    LateNap,
    Rest,
}

#[derive(Deserialize, Debug)]
pub struct Sleep {
    pub id: String,
    pub average_breath: Option<f32>,
    pub average_heart_rate: Option<f32>,
    pub average_hrv: Option<u32>,
    pub awake_time: Option<u32>,
    pub bedtime_end: String,
    pub bedtime_start: String,
    pub day: String,
    pub deep_sleep_duration: Option<u32>,
    pub efficiency: Option<u8>,
    pub heart_rate: Option<Sample>,
    pub hrv: Option<Sample>,
    pub latency: Option<u32>,
    pub light_sleep_duration: Option<u32>,
    pub low_battery_alert: bool,
    pub lowest_heart_rate: Option<u32>,
    pub movement_30_sec: Option<String>,
    pub period: u32,
    pub readiness: Option<ReadinessSummary>,
    pub readiness_score_delta: Option<u8>,
    pub rem_sleep_duration: Option<u32>,
    pub restless_periods: Option<u32>,
    pub sleep_phase_5_min: Option<String>,
    pub sleep_score_delta: Option<u8>,
    pub sleep_algorithm_version: Option<SleepAlgorithmVersion>,
    pub time_in_bed: u32,
    pub total_sleep_duration: Option<u32>,
    pub r#type: SleepType,
}

#[derive(Deserialize, Debug)]
pub struct SleepTimeWindow {
    pub day_tz: u32,
    pub end_offset: u32,
    pub start_offset: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SleepTimeRecommendation {
    ImproveEfficiency,
    EarlierBedtime,
    LaterBedtime,
    EarlierWakeUpTime,
    LaterWakeUpTime,
    FollowOptimalBedtime,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SleepTimeStatus {
    NoteEnoughNights,
    NotEnoughRecentNights,
    BadSleepQuality,
    OnlyRecommendedFound,
    OptimalFound,
}

#[derive(Deserialize, Debug)]
pub struct SleepTime {
    pub id: String,
    pub day: String,
    pub optimal_bedtime: Option<SleepTimeWindow>,
    pub recommendation: Option<SleepTimeRecommendation>,
    pub status: Option<SleepTimeStatus>,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub id: String,
    pub day: String,
    pub text: Option<String>,
    pub timestamp: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutIntensity {
    Easy,
    Moderate,
    Hard,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutSource {
    Manual,
    Autodetected,
    Confirmed,
    WorkoutHeartRate,
}

#[derive(Deserialize, Debug)]
pub struct Workout {
    pub id: String,
    pub activity: String,
    pub calories: Option<f32>,
    pub day: String,
    pub distance: Option<f32>,
    pub end_datetime: String,
    pub intensity: WorkoutIntensity,
    pub label: Option<String>,
    pub source: WorkoutSource,
    pub start_datetime: String,
}

#[derive(Deserialize, Debug)]
pub struct TagV2 {
    pub id: String,
    pub tag_type_code: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub start_day: String,
    pub end_day: Option<String>,
    pub comment: Option<String>,
}

#[derive(Serialize, TypedBuilder)]
pub struct DateQuery<'a> {
    #[builder(default = None, setter(strip_option))]
    start_date: Option<&'a str>,
    #[builder(default = None, setter(strip_option))]
    end_date: Option<&'a str>,
    #[builder(default = None, setter(strip_option))]
    next_token: Option<&'a str>,
}

#[derive(Serialize, TypedBuilder)]
pub struct DatetimeQuery<'a> {
    #[builder(default = None, setter(strip_option))]
    start_datetime: Option<&'a str>,
    #[builder(default = None, setter(strip_option))]
    end_datetime: Option<&'a str>,
    #[builder(default = None, setter(strip_option))]
    next_token: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub next_token: Option<String>,
}

macro_rules! generic_endpoint {
    ($name: ident, $type: ty, $path: literal) => {
        pub fn $name(&self) -> Result<$type, Box<dyn Error>> {
            let url = format!("{}/{}", &self.base_url, $path);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .send()?
                .json::<$type>()?;
            Ok(response)
        }
    };
}

macro_rules! list_endpoint {
    ($name: ident, $type: ty, $path: literal, $params: ty) => {
        pub fn $name(&self, params: $params) -> Result<ListResponse<$type>, Box<dyn Error>> {
            let url = format!("{}/{}", &self.base_url, $path);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .query(&params)
                .send()?
                .json::<ListResponse<$type>>()?;
            Ok(response)
        }
    };
}

macro_rules! get_endpoint {
    ($name: ident, $type: ty, $path: literal) => {
        pub fn $name(&self, id: &str) -> Result<$type, Box<dyn Error>> {
            let url = format!("{}/{}/{}", &self.base_url, $path, id);
            let response = self
                .client
                .get(&url)
                .bearer_auth(&self.token)
                .send()?
                .json::<$type>()?;
            Ok(response)
        }
    };
}

macro_rules! endpoint_set {
    ($name: ident, $type: ty, $path: literal, $params: ty) => {
        paste! {
            get_endpoint!([<get_ $name>], $type, $path);
            list_endpoint!([<list_ $name>], $type, $path, $params);
        }
    };
}

pub struct OuraClient<'a> {
    token: &'a str,
    base_url: &'a str,
    client: Client,
}

impl<'a> OuraClient<'a> {
    pub fn new(token: &'a str) -> Self {
        let client = Client::new();
        Self {
            token,
            client,
            base_url: API_BASE_URL,
        }
    }

    pub fn build_with_base_url(token: &'a str, base_url: &'a str) -> Self {
        let client = Client::new();
        Self {
            token,
            client,
            base_url,
        }
    }

    endpoint_set!(daily_activity, DailyActivity, "daily_activity", DateQuery);

    endpoint_set!(
        daily_readiness,
        DailyReadiness,
        "daily_readiness",
        DateQuery
    );

    endpoint_set!(daily_sleep, DailySleep, "daily_sleep", DateQuery);

    endpoint_set!(daily_spo2, DailySpO2, "daily_spo2", DateQuery);

    list_endpoint!(list_heart_rate, HeartRate, "heartrate", DatetimeQuery);

    generic_endpoint!(get_personal_info, PersonalInfo, "personal_info");

    endpoint_set!(
        rest_mode_period,
        RestModePeriod,
        "rest_mode_period",
        DateQuery
    );

    endpoint_set!(
        ring_configuration,
        RingConfiguration,
        "ring_configuration",
        DateQuery
    );

    endpoint_set!(session, Session, "session", DateQuery);

    endpoint_set!(sleep, Sleep, "sleep", DateQuery);

    endpoint_set!(sleep_time, SleepTime, "sleep_time", DateQuery);

    endpoint_set!(tag, Tag, "tag", DateQuery);

    endpoint_set!(workout, Workout, "workout", DateQuery);

    endpoint_set!(tag_v2, TagV2, "tag/v2", DateQuery);
}
