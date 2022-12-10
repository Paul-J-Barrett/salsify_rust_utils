use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use serde_json;
use serde::{Deserialize,Serialize};

//TODO: move to config file
pub fn get_base_string() -> String {
    format!("https://app.salsify.com/api/orgs/{}/",get_org_id()?.to_str()?)
}

//TODO: move to config file
pub fn get_base_v1_string() -> String {
    format!("https://app.salsify.com/api/v1/orgs/()/",get_org_id()?.to_str()?)
}

struct str_tuple {

}
pub fn get_header() -> (String,String) {
    //pretty simple string not sure if it is necessary to use serde_json yet.
   ("Authorization", format!("Bearer {}",get_token()))
}

pub fn export_run_status(id:String) -> String {
"adsfadsf".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
struct Conditions {
    entity_type:String,
    format:String,
    filter:String,
    properties:String,
    include_all_columns:bool
}

#[derive(Debug, Serialize, Deserialize)]
struct Report_Request {
    conditions:Conditions
}

//{'id': 66404767, 'status': 'running', 'start_time': '2022-12-10T06:00:30.435Z', 'end_time': None, 'duration': 0.160403741, 'url': '', 'progress': 10, 'failure_reason': None, 'estimated_time_remaining': ''}
#[derive(Debug, Serialize, Deserialize)]
struct Report_Response {
    id:usize,
    status:String,
    start_time:String,
    end_time:String,
    duration:String,
    url:String,
    progress:String,
    failure_reason:String,
    estimated_time_remaining:String
}

pub async fn request_report(req:&Report_Request) -> Result<Report_Response,Box<(dyn Error)>> {
    let endpoint = format!("{}export_runs",get_base_string());
    //{'id': 66404767, 'status': 'running', 'start_time': '2022-12-10T06:00:30.435Z', 'end_time': None, 'duration': 0.160403741, 'url': '', 'progress': 10, 'failure_reason': None, 'estimated_time_remaining': ''}

    let res = reqwest::Client::new()
    .post(endpoint)
    .header("Authorization", format!("Bearer {}",get_token()?.to_str()?))
    .json(req)
    .send()
    .await?;

    let result = res
        .json::<Report_Response>()
        .await?;
    
    Ok(result)

}

pub fn get_org_id() ->  String {
    let key="SALSIFYORGID";
    env::var_os(key).unwrap().to_str().unwrap().to_string()
}

pub fn get_token() -> String {
    let key="SALSIFYTOK";
    env::var_os(key).unwrap().to_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_test() {

        assert_eq!(get_token().unwrap().len(),43);
    }

    #[test]
    fn org_id_test() {
        assert_eq!(get_org_id().unwrap().len(),38);
    }
}
