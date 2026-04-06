use crate::config::{AppConfig, get_config_path};
use crate::ledger::{Ledger, LedgerRecord, Stats};
use crate::parser::{parse_input, ParseMode};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub parse_mode: String,
    pub ai_provider: String,
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

/// 解析输入并记录到账本
#[tauri::command]
pub async fn parse_and_record(input: String) -> Result<String, String> {
    // 加载配置
    let config = load_app_config()?;

    // 根据配置选择解析模式
    let mode = if config.parse_mode == "ai" && !config.api_key.is_empty() {
        ParseMode::AI {
            provider: config.ai_provider.clone(),
            api_key: config.api_key.clone(),
            api_base: config.api_base.clone(),
            model: config.model.clone(),
        }
    } else {
        ParseMode::Local
    };

    // 解析输入
    let record = parse_input(&input, mode).await?;

    // 写入账本
    let ledger = Ledger::new()?;
    ledger.append_record(&record)?;

    // 获取统计信息
    let stats = ledger.get_stats("month")?;

    // 返回结果
    let category_total = stats.by_category.get(&record.category).unwrap_or(&0.0);
    Ok(format!(
        "✓ 已记录: {} {} ¥{}\n📊 本月{}累计: ¥{}",
        record.category,
        if record.note.is_empty() { "" } else { &record.note },
        record.amount,
        record.category,
        category_total
    ))
}

/// 获取统计数据
#[tauri::command]
pub async fn get_stats(period: String) -> Result<StatsResponse, String> {
    let ledger = Ledger::new()?;
    let stats = ledger.get_stats(&period)?;

    Ok(StatsResponse {
        total_income: stats.total_income,
        total_expense: stats.total_expense,
        by_category: stats.by_category,
        records: stats.records,
    })
}

/// 获取配置
#[tauri::command]
pub async fn get_config() -> Result<Config, String> {
    let app_config = load_app_config()?;

    Ok(Config {
        parse_mode: app_config.parse_mode,
        ai_provider: app_config.ai_provider,
        api_key: app_config.api_key,
        api_base: app_config.api_base,
        model: app_config.model,
    })
}

/// 保存配置
#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
    let config_path = get_config_path()?;

    let app_config = AppConfig {
        parse_mode: config.parse_mode,
        ai_provider: config.ai_provider,
        api_key: config.api_key,
        api_base: config.api_base,
        model: config.model,
    };

    let json = serde_json::to_string_pretty(&app_config)
        .map_err(|e| e.to_string())?;

    fs::write(&config_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn load_app_config() -> Result<AppConfig, String> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&content)
            .map_err(|e| e.to_string())
    } else {
        Ok(AppConfig::default())
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_income: f64,
    pub total_expense: f64,
    pub by_category: std::collections::HashMap<String, f64>,
    pub records: Vec<LedgerRecordResponse>,
}

#[derive(Debug, Serialize)]
pub struct LedgerRecordResponse {
    pub date: String,
    pub type: String,
    pub category: String,
    pub amount: f64,
    pub note: String,
}

impl From<LedgerRecord> for LedgerRecordResponse {
    fn from(record: LedgerRecord) -> Self {
        LedgerRecordResponse {
            date: record.date.format("%Y-%m-%d").to_string(),
            type: record.record_type,
            category: record.category,
            amount: record.amount,
            note: record.note,
        }
    }
}

impl From<Vec<LedgerRecord>> for Vec<LedgerRecordResponse> {
    fn from(records: Vec<LedgerRecord>) -> Self {
        records.into_iter().map(|r| r.into()).collect()
    }
}