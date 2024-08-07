use reqwest::Url;
use std::io::Read;
use std::time::SystemTime;

pub fn get_blockheight() -> Result<String, &'static str> {
    let _blockheight_no_nl = blockheight().unwrap().to_string();
    Ok(format!("{}", blockheight().unwrap().to_string()))
}

pub fn blockheight() -> Result<f64, &'static str> {
    let since_the_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let seconds = since_the_epoch.as_secs();
    let subsec_millis = since_the_epoch.subsec_millis() as u64;
    let _now_millis = seconds * 1000 + subsec_millis;
    let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut tmp_string = String::new();
    res.read_to_string(&mut tmp_string).unwrap();
    let tmp_u64 = tmp_string.parse::<u64>().unwrap_or(0);
    let blockheight = tmp_u64 as f64;
    return Ok(blockheight);
}
