from __future__ import annotations

import os
from typing import Optional

import pandas as pd


def export_to_excel(
    out_path: str,
    df_summary: pd.DataFrame,
    df_detail: Optional[pd.DataFrame] = None,
    df_need_days: Optional[pd.DataFrame] = None,
) -> str:
    """导出到xlsx文件。

    参数:
        out_path: 输出xlsx路径
        df_summary: 汇总表
        df_detail: 明细表（可选）
        df_need_days: 需要打卡日口径页（可选）
    返回:
        实际输出路径
    """
    os.makedirs(os.path.dirname(os.path.abspath(out_path)), exist_ok=True)
    with pd.ExcelWriter(out_path, engine="openpyxl") as writer:
        df_summary.to_excel(writer, sheet_name="汇总", index=False)
        if df_detail is not None and not df_detail.empty:
            df_detail.to_excel(writer, sheet_name="明细", index=False)
        if df_need_days is not None and not df_need_days.empty:
            df_need_days.to_excel(writer, sheet_name="需要打卡日", index=False)
    return out_path


def export_summary_csv_bom(out_path: str, df_summary: pd.DataFrame) -> str:
    """导出汇总为UTF-8 BOM的CSV。"""
    os.makedirs(os.path.dirname(os.path.abspath(out_path)), exist_ok=True)
    df_summary.to_csv(out_path, index=False, encoding="utf_8_sig")
    return out_path

