from app.processor import parse_excel_files
paths = [r".\\Oct\\10月员工刷卡记录表.xlsx"]
df = parse_excel_files(paths, start_row=None)
print("records:", 0 if df is None else len(df))
print(df.head() if df is not None else 'DF=None')
