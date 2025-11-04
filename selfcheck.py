"""
自检脚本：
 - 构造一个示例Excel文件（三行一人、多日期时刻），
 - 运行解析与聚合，
 - 导出xlsx与csv到当前目录，供人工快速核查。
"""

from __future__ import annotations

import os
import tempfile
import pandas as pd

from app.processor import parse_excel_files, aggregate_daily
from app.exporter import export_to_excel, export_summary_csv_bom


def build_sample_excel(path: str) -> None:
    """构造示例Excel：两位员工，每人一组3行。"""
    # 员工A
    row1_a = ["单位", "姓名：张三", "部门"]
    row2_a = ["1", "2", "3", "4", "5", "6", "7", "8"]
    row3_a = [
        "0835 0910 1205",  # 1日：AM命中(0835)，NOON命中(1205)
        "1100; 1330",      # 2日：NOON命中
        "090459",          # 3日：AM命中(09:04:59)
        "",                # 4日：无
        "8:03",            # 5日：AM命中
        "14:05",           # 6日：越界(>14:04:59)无
        "835/1130",        # 7日：AM命中(08:35)，NOON命中(11:30)
        "10:00"            # 8日：无
    ]

    # 员工B
    row1_b = ["姓名", "李四", "备注"]
    row2_b = [1, 2, 3]
    row3_b = ["07:59\n12:00", "12:30", "0905"]  # 第3日：09:05不计入AM

    df = pd.DataFrame([row1_a, row2_a, row3_a, row1_b, row2_b, row3_b])
    with pd.ExcelWriter(path, engine="openpyxl") as writer:
        df.to_excel(writer, sheet_name="打卡", header=False, index=False)


def main():
    # 创建示例Excel
    sample = os.path.abspath("示例打卡.xlsx")
    build_sample_excel(sample)

    # 解析
    df_long = parse_excel_files([sample])
    print("长表记录数:", len(df_long))
    print(df_long.head())

    # 设定年月与休息日（例如：2024年1月，休息日仅8日）
    y, m = 2024, 1
    rest_days = [8]
    df_detail, df_summary, df_need = aggregate_daily(df_long, y, m, rest_days)

    print("明细示例:")
    print(df_detail.head())
    print("汇总:")
    print(df_summary)

    # 导出
    out_xlsx = os.path.abspath("汇总_自检.xlsx")
    out_csv = os.path.abspath("汇总_自检.csv")
    export_to_excel(out_xlsx, df_summary, df_detail=df_detail, df_need_days=df_need)
    export_summary_csv_bom(out_csv, df_summary)
    print("已导出:", out_xlsx, out_csv)


if __name__ == "__main__":
    main()

