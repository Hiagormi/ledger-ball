use chrono::{DateTime, Local, NaiveDate};
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerRecord {
    pub date: NaiveDate,
    pub record_type: String, // "income" or "expense"
    pub category: String,
    pub amount: f64,
    pub note: String,
}

#[derive(Debug)]
pub struct Stats {
    pub total_income: f64,
    pub total_expense: f64,
    pub by_category: HashMap<String, f64>,
    pub records: Vec<LedgerRecord>,
}

pub struct Ledger {
    data_dir: PathBuf,
}

impl Ledger {
    pub fn new() -> Result<Self, String> {
        // 数据目录在项目文件夹的 data 目录下
        let exe_dir = std::env::current_exe()
            .map_err(|e| format!("无法获取程序目录: {}", e))?
            .parent()
            .ok_or("无法获取父目录")?
            .to_path_buf();

        let data_dir = exe_dir.join("data");

        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)
                .map_err(|e| format!("无法创建数据目录: {}", e))?;
        }

        Ok(Ledger { data_dir })
    }

    /// 获取当月账本文件路径
    fn get_ledger_path(&self) -> PathBuf {
        let now = Local::now();
        let filename = format!("ledger-{}.csv", now.format("%Y-%m"));
        self.data_dir.join(filename)
    }

    /// 检查账本是否存在，不存在则创建
    fn ensure_ledger_exists(&self) -> Result<PathBuf, String> {
        let path = self.get_ledger_path();

        if !path.exists() {
            // 创建新账本文件并写入表头
            let mut writer = WriterBuilder::new()
                .has_headers(true)
                .from_path(&path)
                .map_err(|e| format!("创建账本失败: {}", e))?;

            writer
                .write_record(["日期", "类型", "分类", "金额", "备注"])
                .map_err(|e| format!("写入表头失败: {}", e))?;

            writer.flush().map_err(|e| format!("保存账本失败: {}", e))?;
        }

        Ok(path)
    }

    /// 添加一条记录
    pub fn append_record(&self, record: &LedgerRecord) -> Result<(), String> {
        let path = self.ensure_ledger_exists()?;

        // 追加写入
        let file = OpenOptions::new()
            .append(true)
            .open(&path)
            .map_err(|e| format!("打开账本失败: {}", e))?;

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);

        writer
            .write_record([
                record.date.format("%Y-%m-%d").to_string(),
                record.record_type.clone(),
                record.category.clone(),
                record.amount.to_string(),
                record.note.clone(),
            ])
            .map_err(|e| format!("写入记录失败: {}", e))?;

        writer.flush().map_err(|e| format!("保存记录失败: {}", e))?;

        Ok(())
    }

    /// 获取所有账本文件
    fn get_all_ledger_files(&self) -> Result<Vec<PathBuf>, String> {
        let mut files = Vec::new();

        for entry in fs::read_dir(&self.data_dir)
            .map_err(|e| format!("读取数据目录失败: {}", e))?
        {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "csv")
                && path.file_name().map_or(false, |name| name.to_string_lossy().starts_with("ledger-"))
            {
                files.push(path);
            }
        }

        // 按文件名排序 (最新的在前)
        files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

        Ok(files)
    }

    /// 读取所有记录
    fn read_all_records(&self) -> Result<Vec<LedgerRecord>, String> {
        let files = self.get_all_ledger_files()?;
        let mut records = Vec::new();

        for file in files {
            let mut content = String::new();
            File::open(&file)
                .map_err(|e| format!("打开账本失败: {}", e))?
                .read_to_string(&mut content)
                .map_err(|e| format!("读取账本失败: {}", e))?;

            // 解析 CSV
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(content.as_bytes());

            for result in reader.records() {
                let record = result.map_err(|e| format!("解析记录失败: {}", e))?;

                if record.len() >= 5 {
                    let date = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")
                        .map_err(|e| format!("解析日期失败: {}", e))?;

                    records.push(LedgerRecord {
                        date,
                        record_type: record[1].clone(),
                        category: record[2].clone(),
                        amount: record[3].parse::<f64>()
                            .map_err(|e| format!("解析金额失败: {}", e))?,
                        note: record[4].clone(),
                    });
                }
            }
        }

        Ok(records)
    }

    /// 获取统计数据
    pub fn get_stats(&self, period: &str) -> Result<Stats, String> {
        let all_records = self.read_all_records()?;
        let now = Local::now();

        // 根据周期筛选记录
        let filtered_records: Vec<LedgerRecord> = all_records
            .into_iter()
            .filter(|r| {
                match period {
                    "day" => r.date == now.date_naive(),
                    "week" => {
                        // 本周 (从周一开始)
                        let week_start = now.date_naive()
                            - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
                        r.date >= week_start && r.date <= now.date_naive()
                    }
                    "month" => {
                        r.date.year() == now.year() && r.date.month() == now.month()
                    }
                    "year" => r.date.year() == now.year(),
                    _ => true,
                }
            })
            .collect();

        // 计算统计
        let mut total_income = 0.0;
        let mut total_expense = 0.0;
        let mut by_category: HashMap<String, f64> = HashMap::new();

        for record in &filtered_records {
            if record.record_type == "income" {
                total_income += record.amount;
            } else {
                total_expense += record.amount;

                // 按分类统计支出
                let category_total = by_category.get(&record.category).unwrap_or(&0.0);
                by_category.insert(record.category.clone(), category_total + record.amount);
            }
        }

        Ok(Stats {
            total_income,
            total_expense,
            by_category,
            records: filtered_records,
        })
    }
}