from __future__ import annotations

import re
from dataclasses import dataclass
from datetime import datetime, date, time
from typing import Dict, List, Optional, Tuple

import numpy as np
import pandas as pd


# ==============================
# 数据结构定义
# ==============================

@dataclass
class TimeHit:
    """单日单窗的命中信息（包含原始时间列表）。

    属性:
        has_hit: 是否命中该窗口
        times: 该窗口下的时间字符串列表（标准化为HH:MM:SS）
    """
    has_hit: bool
    times: List[str]


@dataclass
class DailyRecord:
    """逐日的命中汇总信息。"""
    am: TimeHit
    noon: TimeHit

    @property
    def daily_count(self) -> int:
        """当日计次数=AM(0/1)+NOON(0/1)"""
        return int(self.am.has_hit) + int(self.noon.has_hit)

    @property
    def hit_any(self) -> bool:
        """当天任一窗口命中则视为打过卡。"""
        return self.am.has_hit or self.noon.has_hit


# ==============================
# 常量与工具函数
# ==============================

# 时间窗口边界
AM_CUTOFF = time(9, 4, 59)               # AM窗口：<= 09:04:59
NOON_START = time(11, 0, 0)               # NOON窗口：>= 11:00:00
NOON_END = time(14, 4, 59)                # NOON窗口：<= 14:04:59


def _to_int(val) -> Optional[int]:
    """尝试将单元格值转换为整数，如果失败则返回None。"""
    if pd.isna(val):
        return None
    try:
        s = str(val).strip()
        if s == "":
            return None
        return int(float(s))
    except Exception:
        return None


def _norm_time_token(token: str) -> Optional[str]:
    """将一个时间token标准化为HH:MM:SS。

    支持格式：
    - H:MM[:SS] 或 HH:MM[:SS]
    - HMM/HHMM 例如 835/0835/1835
    失败则返回None。
    """
    if token is None:
        return None
    s = str(token).strip()
    if not s:
        return None

    # 1) 带冒号
    m = re.match(r"^(\d{1,2}):(\d{2})(?::(\d{2}))?$", s)
    if m:
        hh = int(m.group(1))
        mm = int(m.group(2))
        ss = int(m.group(3)) if m.group(3) is not None else 0
        if 0 <= hh <= 23 and 0 <= mm <= 59 and 0 <= ss <= 59:
            return f"{hh:02d}:{mm:02d}:{ss:02d}"
        return None

    # 2) 紧凑数字 HMM/HHMM
    m = re.match(r"^(\d{3,4})$", s)
    if m:
        num = m.group(1)
        if len(num) == 3:
            hh = int(num[0])
            mm = int(num[1:])
        else:
            hh = int(num[:2])
            mm = int(num[2:])
        if 0 <= hh <= 23 and 0 <= mm <= 59:
            return f"{hh:02d}:{mm:02d}:00"
        return None

    return None


def _which_window(t: time) -> Optional[str]:
    """判断时间属于哪个窗口。

    返回"AM"/"NOON"/None。
    """
    if t <= AM_CUTOFF:
        return "AM"
    if NOON_START <= t <= NOON_END:
        return "NOON"
    return None


def month_days(year: int, month: int) -> int:
    """返回该月天数。"""
    if month == 12:
        next_first = date(year + 1, 1, 1)
    else:
        next_first = date(year, month + 1, 1)
    this_first = date(year, month, 1)
    return (next_first - this_first).days


# ==============================
# 解析原表
# ==============================

def _extract_name_from_row(row_values: List) -> Optional[str]:
    """从第1行数据中提取姓名。

    优先匹配“姓名：XXX”；否则取“姓名”所在单元格右侧第一个非空值。
    """
    # 先查带冒号的形式
    for val in row_values:
        if pd.isna(val):
            continue
        s = str(val).strip()
        if s.startswith("姓名：") or s.startswith("姓名:"):
            name = s.split("：", 1)[-1].split(":", 1)[-1].strip()
            if name:
                return name

    # 再查“姓名”+右侧非空
    for idx, val in enumerate(row_values):
        if pd.isna(val):
            continue
        s = str(val).strip()
        if s == "姓名":
            # 向右搜索第一个非空
            for j in range(idx + 1, len(row_values)):
                v = row_values[j]
                if not pd.isna(v) and str(v).strip() != "":
                    return str(v).strip()
            break

    return None


def _parse_person_block(sheet_df: pd.DataFrame, start_row: int) -> Tuple[Optional[Dict], int]:
    """尝试从start_row起解析一个人（3行）。

    返回(结果或None, 下一行索引)。
    结果结构：
    {
      'name': str,
      'day_to_tokens': {day:int -> List[str]},
    }
    """
    n_rows = sheet_df.shape[0]
    if start_row + 2 >= n_rows:
        return None, start_row + 1

    row1 = sheet_df.iloc[start_row].tolist()
    row2 = sheet_df.iloc[start_row + 1].tolist()
    row3 = sheet_df.iloc[start_row + 2].tolist()

    name = _extract_name_from_row(row1)
    if not name:
        return None, start_row + 1

    # 日期行：收集各列对应的日号
    col_to_day: Dict[int, int] = {}
    for c, v in enumerate(row2):
        day = _to_int(v)
        if day is not None and 1 <= day <= 31:
            col_to_day[c] = day

    if not col_to_day:
        # 没有日期映射，视为失败
        return None, start_row + 1

    # 第3行：各列拆分时刻
    sep = re.compile(r"[\s,;/\\]+")  # 空格/逗号/分号/斜杠/反斜杠/换行
    day_to_tokens: Dict[int, List[str]] = {}
    for c, v in enumerate(row3):
        if c not in col_to_day:
            continue
        d = col_to_day[c]
        if pd.isna(v):
            continue
        s = str(v)
        tokens = [t for t in sep.split(s) if t and t.strip()]
        if not tokens:
            continue
        day_to_tokens.setdefault(d, []).extend(tokens)

    return {"name": name, "day_to_tokens": day_to_tokens}, start_row + 3


def parse_excel_files(file_paths: List[str]) -> pd.DataFrame:
    """读取多个Excel文件并解析成长表结构。

    返回DataFrame列：['姓名','日','时间','窗口','源文件','工作表']
    其中：时间为标准化HH:MM:SS；窗口为'AM'/'NOON'/None（无效时间不产出记录）。
    """
    records: List[Tuple[str, int, str, str, str, str]] = []

    for f in file_paths:
        try:
            xls = pd.ExcelFile(f)
        except Exception:
            continue

        for sheet_name in xls.sheet_names:
            try:
                df = pd.read_excel(f, sheet_name=sheet_name, header=None, dtype=object)
            except Exception:
                continue

            r = 0
            while r < df.shape[0]:
                res, next_r = _parse_person_block(df, r)
                if res is None:
                    r += 1
                    continue
                name = res["name"]
                day_to_tokens = res["day_to_tokens"]

                for d, tokens in day_to_tokens.items():
                    for tok in tokens:
                        norm = _norm_time_token(tok)
                        if not norm:
                            continue
                        hh, mm, ss = [int(x) for x in norm.split(":")]
                        t = time(hh, mm, ss)
                        w = _which_window(t)
                        if not w:
                            continue
                        records.append((name, d, norm, w, f, sheet_name))

                r = next_r

    if not records:
        return pd.DataFrame(columns=["姓名", "日", "时间", "窗口", "源文件", "工作表"])

    df_long = pd.DataFrame(records, columns=["姓名", "日", "时间", "窗口", "源文件", "工作表"])
    return df_long


# ==============================
# 聚合与汇总
# ==============================

def aggregate_daily(df_long: pd.DataFrame, year: int, month: int, rest_days: List[int]) -> Tuple[pd.DataFrame, pd.DataFrame, pd.DataFrame]:
    """根据长表聚合生成明细与汇总。

    参数:
        df_long: parse_excel_files输出
        year, month: 年月
        rest_days: 休息日(1..31)

    返回:
        df_detail: 按姓名-日期的明细（含AM/NOON命中与时间列表）
        df_summary: 按姓名的汇总
        df_need_days: 需要打卡日页面数据
    """
    mdays = month_days(year, month)
    rest_set = set([d for d in rest_days if 1 <= d <= mdays])
    need_days = [d for d in range(1, mdays + 1) if d not in rest_set]

    # 构建逐人逐日窗口聚合
    results_detail: List[Dict] = []
    for name, sub in df_long.groupby("姓名"):
        # 先按日期聚合窗口
        day_map: Dict[int, DailyRecord] = {}
        for d, g in sub.groupby("日"):
            # 每日窗口去重
            am_times: List[str] = sorted({t for t, w in zip(g["时间"], g["窗口"]) if w == "AM"})
            noon_times: List[str] = sorted({t for t, w in zip(g["时间"], g["窗口"]) if w == "NOON"})
            dr = DailyRecord(
                am=TimeHit(bool(am_times), am_times),
                noon=TimeHit(bool(noon_times), noon_times),
            )
            day_map[int(d)] = dr

        # 输出需要打卡日范围内的明细
        for d in need_days:
            dr = day_map.get(d, DailyRecord(TimeHit(False, []), TimeHit(False, [])))
            results_detail.append({
                "姓名": name,
                "日期": f"{year:04d}-{month:02d}-{d:02d}",
                "日": d,
                "AM命中": int(dr.am.has_hit),
                "NOON命中": int(dr.noon.has_hit),
                "当日计次": dr.daily_count,
                "AM时间列表": ", ".join(dr.am.times),
                "NOON时间列表": ", ".join(dr.noon.times),
            })

    df_detail = pd.DataFrame(results_detail, columns=[
        "姓名", "日期", "日", "AM命中", "NOON命中", "当日计次", "AM时间列表", "NOON时间列表"
    ])

    # 汇总
    summaries: List[Dict] = []
    for name, sub in df_detail.groupby("姓名"):
        punch_days = int((sub["当日计次"] > 0).sum())
        punch_count = int(sub["当日计次"].sum())
        need_day_count = len(need_days)
        expect_count = 2 * need_day_count

        # 缺勤具体日期：仅列0次的那些需要打卡日
        missing_dates = sub.loc[sub["当日计次"] == 0, "日"].tolist()
        missing_days = len(missing_dates)
        absent_count = expect_count - punch_count

        summaries.append({
            "姓名": name,
            "需要打卡日": need_day_count,
            "应打卡次数": expect_count,
            "打卡天数": punch_days,
            "打卡次数": punch_count,
            "缺勤天数": missing_days,
            "缺勤次数": absent_count,
            "缺勤具体日期": ",".join(str(d) for d in missing_dates),
        })

    df_summary = pd.DataFrame(summaries, columns=[
        "姓名", "需要打卡日", "应打卡次数", "打卡天数", "打卡次数", "缺勤天数", "缺勤次数", "缺勤具体日期"
    ])

    # 需要打卡日口径页
    df_need_days = pd.DataFrame({
        "年份":[year]*len(need_days),
        "月份":[month]*len(need_days),
        "需要打卡日": need_days,
        "休息日": [np.nan]*len(need_days),
    })

    return df_detail, df_summary, df_need_days

