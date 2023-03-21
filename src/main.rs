extern crate clap;
use clap::Parser;
extern crate anyhow;
use anyhow::Result;
extern crate pretty_env_logger;
#[macro_use] extern crate log;
extern crate reqwest;
use reqwest::Url;

extern crate json;

#[derive(Parser,Debug)]
struct Args {
    brand: String,
    query: Option<String>,
    #[arg(short,long,default_value_t=100)]
    step: usize,
}


fn create_url(brand: impl AsRef<str>, query: &Option<String>, page: usize, step: usize) -> Result<Url> {
    let q = match query {
        Some(qref) => String::from(qref),
        _ => String::from(""),
    };

    let urlstr = format!("https://poshmark.com/vm-rest/posts?request={{\"filters\":{{\"department\":\"All\",\"brand\":[\"{}\"],\"inventory_status\":[\"available\"]}},\"query_and_facet_filters\":{{\"department\":\"All\"}},\"query\":\"{}\",\"experience\":\"all\",\"sizeSystem\":\"us\",\"max_id\":\"{}\",\"count\":\"{}\"}}&summarize=true&feature_extraction_setting=null&suggested_filters_count=40&end_of_search=false&disable_fallback=false&summarize=true&pm_version=236.0.0",
    brand.as_ref(), q, page, step);

    let url = Url::parse(&urlstr)?;


    Ok(url)
}


fn count_results(brand: impl AsRef<str>, query: &Option<String>, page: usize, step: usize) -> Result<usize> {
    let url = create_url(brand, query, page, step)?;
    debug!("URL created: {}", url);
    let client = reqwest::blocking::Client::new();
    let res = client.get(url.clone())
        .body("")
        .send()?;
    if res.status().is_success() {
        debug!("Request successful.");
    }
    else if res.status().is_server_error() {
        error!("Server error occurred.");
    }
    else {
        info!("HTTP response status: {:?}", res.status());
    }
    let text = res.text()?;
    trace!("Result: {:?}",&text);
    let parsed_json = json::parse(&text)?;
    let data_len = parsed_json["data"].len();
    info!("Number of results for page {}: {}", &page, &data_len);
    if data_len == 0 {
        warn!("Data length 0 found for page {}.", &page);
    }
    Ok(data_len)
}
fn main() -> Result<()> {
    pretty_env_logger::try_init()?;
    let args = Args::parse();
    run(args).map_err(|e| { error!("{}",e); e })?;
    Ok(())
}
fn run(args: Args) -> Result<()> {
    let brand = args.brand;
    let query = args.query;
    let step = args.step;
    let mut page: usize = 1;
    let mut done = false;
    let mut sum = 0;
    while !done {
        let count = count_results(&brand, &query, page, step)?;
        if count == 0 {
            done = true;
        }
        else {
            sum += count;
            page += 1;
        }
    }
    info!("Total number of results: {}", sum);
    println!("{}",sum);
    Ok(())
}

