use anyhow::Result;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::env;
use std::io;
use std::{thread, time};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SalsifyError {
    #[error("Request Error From API")]
    RequestError,
    #[error("Token not set in Environment")]
    MissingToken,
    #[error("Orginazation ID not set in Environment")]
    MissingOrgID,
    #[error("IO Error")]
    IOError {
        #[from]
        source: io::Error,
    },
}

//TODO: move to config file
///Return Salsify base url.
pub fn get_base_string() -> String {
    format!("https://app.salsify.com/api/orgs/{}/", get_org_id())
}

//TODO: move to config file
///Return Salsify V1 base url.
pub fn get_base_v1_string() -> String {
    format!("https://app.salsify.com/api/v1/orgs/{}/", get_org_id())
}

///Get SALSIFYORGID from Environment
pub fn get_org_id() -> String {
    let key = "SALSIFYORGID";
    env::var_os(key).unwrap().to_str().unwrap().to_string()
}

///Get SALSIFYTOK from Environment
pub fn get_token() -> String {
    let key = "SALSIFYTOK";
    env::var_os(key).unwrap().to_str().unwrap().to_string()
}

///Valid Salsify entity type values for API

#[derive(Default, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SALSIFY_ENTITY_TYPE {
    all,
    #[default]
    product,
    attribute,
    attribute_value,
    digital_asset,
}

///Valid Salsify formats to return for API
#[derive(Default, Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SALSIFY_FORMAT {
    json,
    #[default]
    csv,
    xslx,
    jsonl,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Conditions {
    pub entity_type: String,
    pub format: String,
    pub filter: String,
    pub properties: String,
    pub include_all_columns: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ReportRequest {
    pub conditions: Conditions,
}

//{'id': 66404767, 'status': 'running', 'start_time': '2022-12-10T06:00:30.435Z', 'end_time': None, 'duration': 0.160403741, 'url': '', 'progress': 10, 'failure_reason': None, 'estimated_time_remaining': ''}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ReportResponse {
    pub id: String,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
    pub duration: String,
    pub url: String,
    pub progress: String,
    pub includes_changes_before: String,
    pub failure_reason: String,
    pub estimated_time_remaining: String,
}

pub async fn request_report(req: &ReportRequest) -> Result<ReportResponse> {
    let endpoint = format!("{}export_runs", get_base_string());
    //{'id': 66404767, 'status': 'running', 'start_time': '2022-12-10T06:00:30.435Z', 'end_time': None, 'duration': 0.160403741, 'url': '', 'progress': 10, 'failure_reason': None, 'estimated_time_remaining': ''}
    info!("{}", endpoint);
    let res = reqwest::Client::new()
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", get_token()))
        .json(req)
        .send()
        .await?;

    debug!("{}", res.status());

    let result = res.json::<ReportResponse>().await?;
    debug!("{}", result.status);

    Ok(result)
}

pub fn sleep_secs(secs: f32) {
    let seconds = time::Duration::from_secs_f32(secs);
    thread::sleep(seconds);
}

///Check Status of report and get download url
pub async fn export_run_status(id: String) -> Result<ReportResponse> {
    let endpoint = format!("{}export_runs/{}", get_base_string(), id);
    info!("{}", endpoint);
    let res = reqwest::Client::new()
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", get_token()))
        .send()
        .await?;

    debug!("{}", res.status());

    let result = res.json::<ReportResponse>().await?;
    debug!("{}", result.status);
    Ok(result)
}

///Fetch report from url.
pub async fn get_report(url: String) -> Result<String> {
    let file = reqwest::Client::new().get(url).send().await?;
    let results = file.text().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_test() {
        assert_eq!(get_token().len(), 43);
    }

    #[test]
    fn org_id_test() {
        assert_eq!(get_org_id().len(), 38);
    }
}
