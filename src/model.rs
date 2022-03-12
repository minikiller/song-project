use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Body {
    pub id: i32,                     //自动增加的id
    pub content: String,             //采集的数据，用户端提交的异常数据，不做内容解析
    pub device_id: i32,              //设备id
    pub time: Option<NaiveDateTime>, //写入的时间
}
