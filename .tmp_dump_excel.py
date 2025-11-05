import pandas as pd
import numpy as np
path = r".\\Oct\\10月员工刷卡记录表.xlsx"
xls = pd.ExcelFile(path, engine="openpyxl")
for sh in xls.sheet_names:
    df = pd.read_excel(path, sheet_name=sh, header=None, dtype=object, engine="openpyxl")
    print("SHEET:", sh, "shape=", df.shape)
    # 打印前40行，每行转为字符串，去除nan
    maxr = min(60, df.shape[0])
    for i in range(maxr):
        row = df.iloc[i].tolist()
        # 清理None/NaN
        row_s = ["" if (x is None or (isinstance(x,float) and pd.isna(x))) else str(x) for x in row]
        # 只显示前80列
        row_s = row_s[:80]
        print(f"ROW {i+1:02d}:", " | ".join(row_s))
    print("----")
