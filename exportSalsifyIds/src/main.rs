#![allow(non_snake_case)]
use std::error::Error;

use salsify_lib::{Conditions,Report_Request,Report_Response};

#[tokio::main]
async fn main() ->Result<(),Box<dyn Error>> {

    println!("Hello, Salsify User!");
    println!("Org ID:{:?}",testlib::get_org_id());
    println!("Token:{:?}",testlib::get_token());
    let tok:String = testlib::get_token().unwrap().to_str().unwrap().to_string();
    let orgid=testlib::get_org_id().unwrap().to_str().unwrap().to_string();
    println!("Token length: {}, OrgID Len: {}",tok.len(), orgid.len());

    tes



    Ok(())
}
