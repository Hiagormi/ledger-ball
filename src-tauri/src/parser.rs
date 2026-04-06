use crate::ledger::LedgerRecord;
use chrono::{Local, NaiveDate};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub enum ParseMode {
    Local,
    AI {
        provider: String,
        api_key: String,
        api_base: String,
        model: String,
    },
}

/// 解析用户输入，返回账本记录
pub async fn parse_input(input: &str, mode: ParseMode) -> Result<LedgerRecord, String> {
    match mode {
        ParseMode::Local => parse_local(input),
        ParseMode::AI { provider, api_key, api_base, model } => {
            parse_with_ai(input, &provider, &api_key, &api_base, &model).await
        }
    }
}

/// 本地规则解析
fn parse_local(input: &str) -> Result<LedgerRecord, String> {
    let input = input.trim();

    // 常见分类
    let categories = [
        "餐饮", "交通", "购物", "娱乐", "医疗", "教育",
        "住房", "通讯", "水电", "日用", "其他", "收入",
        "工资", "兼职", "红包", "理财"
    ];

    // 解析日期 (支持: 3/15, 03-15, 2024-03-15)
    let date_regex = Regex::new(r"(\d{4}[-/]\d{1,2}[-/]\d{1,2})|(\d{1,2}[-/]\d{1,2})").unwrap();

    let mut date = Local::now().date_naive();
    let mut remaining = input.to_string();

    if let Some(m) = date_regex.find(&remaining) {
        let date_str = m.as_str();

        // 尝试解析日期
        if date_str.contains("-") || date_str.contains("/") {
            // 尝试完整日期格式
            if let Ok(d) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                date = d;
            } else if let Ok(d) = NaiveDate::parse_from_str(date_str, "%Y/%m/%d") {
                date = d;
            } else if let Ok(d) = NaiveDate::parse_from_str(
                &format!("{}-{}", Local::now().year(), date_str),
                "%Y-%m-%d"
            ) {
                date = d;
            } else if let Ok(d) = NaiveDate::parse_from_str(
                &format!("{}-{}", Local::now().year(), date_str.replace("/", "-")),
                "%Y-%m-%d"
            ) {
                date = d;
            }
        }

        remaining = remaining.replace(date_str, "");
    }

    remaining = remaining.trim().to_string();

    // 判断收入/支出
    let (record_type, remaining) = if remaining.contains("+")
        || remaining.contains("收入")
        || remaining.contains("工资")
        || remaining.contains("兼职")
        || remaining.contains("红包")
        || remaining.contains("理财")
    {
        ("income", remaining.replace("+", "").replace("收入", "").trim().to_string())
    } else {
        ("expense", remaining)
    };

    // 解析金额 (支持: 35, 35元, ￥35, ¥35)
    let amount_regex = Regex::new(r"[¥￥]?\s*(\d+(?:\.\d{1,2})?)\s*(元?)").unwrap();

    let mut amount = 0.0;
    let mut remaining_after_amount = remaining.clone();

    if let Some(m) = amount_regex.find(&remaining) {
        let amount_str = m.as_str();
        // 提取数字
        let num_regex = Regex::new(r"\d+(?:\.\d{1,2})?").unwrap();
        if let Some(num_match) = num_regex.find(amount_str) {
            amount = num_match.as_str().parse::<f64>()
                .map_err(|e| format!("解析金额失败: {}", e))?;
        }
        remaining_after_amount = remaining.replace(amount_str, "").trim().to_string();
    } else {
        return Err("未找到金额，请使用格式如: 餐饮 35 或 午饭 35元".to_string());
    }

    // 解析分类
    let mut category = "其他".to_string();
    let mut note = String::new();

    for cat in categories {
        if remaining_after_amount.contains(cat) {
            category = cat.to_string();
            remaining_after_amount = remaining_after_amount.replace(cat, "").trim().to_string();
            break;
        }
    }

    // 如果分类是收入相关的，强制设为收入
    if category == "工资" || category == "兼职" || category == "红包" || category == "理财" {
        category = "收入".to_string();
    }

    // 剩余部分作为备注
    if !remaining_after_amount.is_empty() {
        note = remaining_after_amount;
    }

    // 如果没有找到分类，尝试从备注推断
    if category == "其他" && !note.is_empty() {
        let hints: HashMap<&str, &str> = [
            ("饭", "餐饮"), ("吃", "餐饮"), ("餐", "餐饮"), ("麦当劳", "餐饮"),
            ("肯德基", "餐饮"), ("外卖", "餐饮"), ("超市", "购物"),
            ("买", "购物"), ("购物", "购物"), ("打车", "交通"),
            ("地铁", "交通"), ("公交", "交通"), ("加油", "交通"),
            ("电影", "娱乐"), ("游戏", "娱乐"), ("KTV", "娱乐"),
        ].iter().cloned().collect();

        for (hint, cat) in hints {
            if note.contains(hint) {
                category = cat.to_string();
                break;
            }
        }
    }

    Ok(LedgerRecord {
        date,
        record_type: record_type.to_string(),
        category,
        amount,
        note,
    })
}

/// AI 解析
async fn parse_with_ai(
    input: &str,
    provider: &str,
    api_key: &str,
    api_base: &str,
    model: &str,
) -> Result<LedgerRecord, String> {
    #[derive(Debug, Serialize)]
    struct Message {
        role: String,
        content: String,
    }

    #[derive(Debug, Serialize)]
    struct RequestBody {
        model: String,
        messages: Vec<Message>,
        temperature: f32,
    }

    #[derive(Debug, Deserialize)]
    struct ResponseBody {
        choices: Vec<Choice>,
    }

    #[derive(Debug, Deserialize)]
    struct Choice {
        message: MessageContent,
    }

    #[derive(Debug, Deserialize)]
    struct MessageContent {
        content: String,
    }

    #[derive(Debug, Deserialize)]
    struct ParsedRecord {
        #[serde(rename = "type")]
        record_type: String,
        category: String,
        amount: f64,
        note: Option<String>,
        date: Option<String>,
    }

    let prompt = format!(
        "请解析以下记账输入，返回JSON格式:\n\
        输入: \"{}\"\n\n\
        要求返回JSON:\n\
        {{\n\
          \"type\": \"income\" 或 \"expense\",\n\
          \"category\": 分类名称 (如: 餐饮/交通/购物/娱乐/医疗/教育/住房/通讯/水电/日用/其他/收入),\n\
          \"amount\": 金额数字,\n\
          \"note\": 备注说明,\n\
          \"date\": \"YYYY-MM-DD\" 格式的日期 (可选，默认今天)\n\
        }}\n\n\
        只返回JSON，不要其他内容。",
        input
    );

    let body = RequestBody {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "你是一个记账助手，帮助用户解析记账输入并返回结构化数据。".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.1,
    };

    let client = reqwest::Client::new();

    let url = format!("{}/chat/completions", api_base);

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求AI服务失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("AI服务返回错误: {} - {}", status, text));
    }

    let response_body: ResponseBody = response
        .json()
        .await
        .map_err(|e| format!("解析AI响应失败: {}", e))?;

    let content = response_body
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or("AI未返回有效响应")?;

    // 清理可能的markdown代码块标记
    let content = content
        .replace("```json", "")
        .replace("```", "")
        .trim()
        .to_string();

    let parsed: ParsedRecord = serde_json::from_str(&content)
        .map_err(|e| format!("解析AI返回的JSON失败: {} - 响应内容: {}", e, content))?;

    let date = if let Some(d) = parsed.date {
        NaiveDate::parse_from_str(&d, "%Y-%m-%d")
            .map_err(|e| format!("解析日期失败: {}", e))?
    } else {
        Local::now().date_naive()
    };

    Ok(LedgerRecord {
        date,
        record_type: parsed.record_type,
        category: parsed.category,
        amount: parsed.amount,
        note: parsed.note.unwrap_or_default(),
    })
}