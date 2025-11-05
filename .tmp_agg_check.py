from app.processor import parse_excel_files, aggregate_daily
paths = [r".\\Oct\\10月员工刷卡记录表.xlsx"]
df = parse_excel_files(paths)
print('len df', len(df))
# assume month 10, year 2025 per header; but we don't parse header; user selects; pick 2025-10
import pandas as pd

y,m = 2025,10
rest=[]

df_detail, df_summary, df_need = aggregate_daily(df, y, m, rest)
print(df_summary.head())
print('detail rows', len(df_detail))
