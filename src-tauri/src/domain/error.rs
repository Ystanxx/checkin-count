use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("非法日期: {0}")]
    InvalidDay(u32),
    #[error("非法时间 token: {0}")]
    InvalidTimeToken(String),
    #[error("规则非法: {0}")]
    InvalidRule(String),
    #[error("人员块解析失败: {0}")]
    BlockParse(String),
    #[error("年份或月份非法: {0}")]
    YearMonth(String),
}
