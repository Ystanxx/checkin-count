from app.processor import parse_excel_files, aggregate_daily, parse_all_names
paths = [r".\\Oct\\10月员工刷卡记录表.xlsx"]
df = parse_excel_files(paths)
names_all = parse_all_names(paths)
print('records:', len(df), 'names:', len(names_all))

y,m,rests = 2025,10,[]
df_detail, df_summary, df_need = aggregate_daily(df, y, m, rests, names_all=names_all)
print('summary rows', len(df_summary))
print(df_summary.head())
