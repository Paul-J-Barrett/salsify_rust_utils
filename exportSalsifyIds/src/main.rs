#![allow(non_snake_case)]
use anyhow::Result;
use log::{debug, info};
use salsify_lib;

#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let cond = salsify_lib::Conditions {
        entity_type: format!("{:?}", salsify_lib::SALSIFY_ENTITY_TYPE::product),
        format: format!("{:?}", salsify_lib::SALSIFY_FORMAT::csv),
        filter: "".into(),
        properties: "'SKU'".into(),
        include_all_columns: false,
    };

    info!("{:?}", cond);

    let req = salsify_lib::ReportRequest { conditions: cond };

    let result = salsify_lib::request_report(&req).await?;
    info!("{}", result.id);
    salsify_lib::sleep_secs(10.0);
    loop {
        let result = salsify_lib::export_run_status(result.id.to_string()).await?;
        debug!("time remaining{}", result.estimated_time_remaining);
        if result.status == "complete" {
            let file = salsify_lib::get_report(result.url).await?;
            debug!("{:?}", file.as_str());
            break;
        }
        salsify_lib::sleep_secs(10.0)
    }

    Ok(())
}
