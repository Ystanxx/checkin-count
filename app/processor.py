from __future__ import annotations

import re
from dataclasses import dataclass
from datetime import datetime, date, time
from typing import Dict, List, Optional, Tuple, Union

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

# 时间窗口边界（常量）
AM_CUTOFF = time(9, 4, 59)               # AM窗口：<= 09:04:59
NOON_START = time(11, 0, 0)               # NOON窗口：>= 11:00:00
NOON_END = time(14, 4, 59)                # NOON窗口：<= 14:04:59


def _to_day(val) -> Optional[int]:
    """从单元格值中更鲁棒地提取日号(1..31)。

    兼容：全角数字、带"日"后缀或其他非数字装饰。
    """
    if pd.isna(val):
        return None
    s = str(val).strip()
    if not s:
        return None
    s = _to_ascii_fullwidth(s)
    m = re.match(r"^\D*(\d{1,2})\D*$", s)
    if not m:
        return None
    d = int(m.group(1))
    if 1 <= d <= 31:
        return d
    return None


def _to_ascii_fullwidth(s: str) -> str:
    """将常见全角数字与标点转为半角ASCII。"""
    if not s:
        return s
    # 全角数字 ０-９, 冒号：，逗号，分号；，斜杠／，空格　
    trans = {
        ord("０"): "0", ord("１"): "1", ord("２"): "2", ord("３"): "3", ord("４"): "4",
        ord("５"): "5", ord("６"): "6", ord("７"): "7", ord("８"): "8", ord("９"): "9",
        ord("："): ":", ord("，"): ",", ord("；"): ";", ord("／"): "/", ord("、"): ",",
        ord("　"): " ",
    }
    return s.translate(trans)


def _norm_time_token(token: Union[str, int, float, time, datetime, pd.Timestamp]) -> Optional[str]:
    """将一个时间token标准化为HH:MM:SS。

    支持格式：
    - H:MM[:SS] 或 HH:MM[:SS]
    - HMM/HHMM 例如 835/0835/1835
    失败则返回None。
    """
    if token is None or (isinstance(token, float) and pd.isna(token)):
        return None

    # 直接处理时间型
    if isinstance(token, time):
        return f"{token.hour:02d}:{token.minute:02d}:{token.second:02d}"
    if isinstance(token, (datetime, pd.Timestamp)):
        t = token.time()
        return f"{t.hour:02d}:{t.minute:02d}:{t.second:02d}"

    # 处理纯数字（Excel可能读成int/float，如903/905）
    if isinstance(token, (int,)) or (isinstance(token, float) and token.is_integer()):
        n = int(token)
        if 0 <= n <= 2359:
            if n < 100:  # 例如 5 -> 00:05:00，不太像有效数据，忽略
                return None
            if n < 1000:
                hh = n // 100
                mm = n % 100
            else:
                hh = n // 100
                mm = n % 100
            if 0 <= hh <= 23 and 0 <= mm <= 59:
                return f"{hh:02d}:{mm:02d}:00"
        return None

    # 处理Excel时间序列（float分数，0~1代表一天的小数）
    if isinstance(token, float) and 0 < token < 1:
        total_seconds = int(round(token * 24 * 3600))
        hh = (total_seconds // 3600) % 24
        mm = (total_seconds % 3600) // 60
        ss = total_seconds % 60
        return f"{hh:02d}:{mm:02d}:{ss:02d}"

    s = str(token).strip()
    if not s:
        return None
    s = _to_ascii_fullwidth(s)

    # 1) 带冒号
    m = re.match(r"^(\d{1,2}):(\d{1,2})(?::(\d{1,2}))?$", s)
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
    # 先查带冒号/含内容的单格（鲁棒：允许“姓 名”“：/:”与可选空格）
    name_regex = re.compile(r"姓\s*名\s*[：:]\s*([^\s:：\-\|]+)")
    for val in row_values:
        if pd.isna(val):
            continue
        s = _to_ascii_fullwidth(str(val)).strip()
        m = name_regex.search(s)
        if m:
            name = m.group(1).strip()
            if name:
                return name

    # 再查“姓名/姓名：”+右侧非空
    for idx, val in enumerate(row_values):
        if pd.isna(val):
            continue
        s = _to_ascii_fullwidth(str(val)).strip()
        s_clean = s.replace("：", "").replace(":", "").strip()
        if s_clean == "姓名":
            # 向右搜索第一个非空
            for j in range(idx + 1, len(row_values)):
                v = row_values[j]
                if not pd.isna(v) and str(v).strip() != "":
                    return str(v).strip()
            break

    return None


def _parse_person_block(sheet_df: pd.DataFrame, start_row: int) -> Tuple[Optional[Dict], int]:
    """尝试从start_row起解析一个人块（姓名行 + 日期行 + 时刻行）。

    更加健壮：允许姓名行后的1~3行内出现日期行，适配含“工号：x”等额外行的表格。

    返回(结果或None, 下一行索引)。
    结果结构：
    {
      'name': str,
      'day_to_tokens': {day:int -> List[str]},
    }
    """
    n_rows = sheet_df.shape[0]
    if start_row >= n_rows:
        return None, start_row + 1

    row1 = sheet_df.iloc[start_row].tolist()
    name = _extract_name_from_row(row1)
    if not name:
        return None, start_row + 1

    # 在姓名行之后的1~3行内寻找“日期行”
    day_row_idx = None
    col_to_day: Dict[int, int] = {}
    for off in (1, 2, 3):
        idx = start_row + off
        if idx >= n_rows:
            break
        row_candidate = sheet_df.iloc[idx].tolist()
        tmp_map: Dict[int, int] = {}
        for c, v in enumerate(row_candidate):
            day = _to_day(v)
            if day is not None and 1 <= day <= 31:
                tmp_map[c] = day
        if len(tmp_map) >= 5:  # 至少识别到若干天数，判定为日期行
            day_row_idx = idx
            col_to_day = tmp_map
            break

    if day_row_idx is None:
        return None, start_row + 1

    # 时刻行：允许合并接下来的多行，直到遇到下一个块的迹象
    sep = re.compile(r"[\s,，,;；/／\\、\-\–\—()（）]+")  # 增强分隔符：含全角、连字符、括号
    day_to_tokens: Dict[int, List[str]] = {}
    time_row_idx = day_row_idx + 1
    cur = time_row_idx
    max_follow = 3  # 最多合并3行
    while cur < n_rows and cur <= time_row_idx + max_follow:
        row_time = sheet_df.iloc[cur].tolist()

        # 碰到下一位人员的姓名/工号提示则停止
        joined = " ".join([str(x) for x in row_time if not pd.isna(x)])
        if "姓名" in joined or "工号" in joined:
            break

        # 如果该行看起来又是日期行（>=5个日号），也停止
        count_days = 0
        for v in row_time:
            dtmp = _to_day(v)
            if dtmp is not None and 1 <= dtmp <= 31:
                count_days += 1
        if count_days >= 5:
            break

        # 解析本行时间（支持“日号列→下一日号列之间”为同一日的列范围）
        n_cols = sheet_df.shape[1]
        if col_to_day:
            # 构建列到日的全覆盖映射
            sorted_cols = sorted(col_to_day.items(), key=lambda x: x[0])
            col_day_full: Dict[int, int] = {}
            for i, (c_start, dval) in enumerate(sorted_cols):
                c_end = (sorted_cols[i + 1][0] - 1) if i + 1 < len(sorted_cols) else (n_cols - 1)
                for cc in range(c_start, c_end + 1):
                    col_day_full[cc] = dval
        else:
            col_day_full = {}

        for c, v in enumerate(row_time):
            if c not in col_day_full:
                continue
            d = col_day_full[c]
            if pd.isna(v):
                continue
            s = _to_ascii_fullwidth(str(v))
            tokens = [t for t in sep.split(s) if t and t.strip()]
            if not tokens:
                continue
            day_to_tokens.setdefault(d, []).extend(tokens)

        cur += 1

    return {"name": name, "day_to_tokens": day_to_tokens}, cur


def parse_excel_files(file_paths: List[str], start_row: Optional[int] = None) -> pd.DataFrame:
    """读取多个Excel文件并解析成长表结构。

    返回DataFrame列：['姓名','日','时间','窗口','源文件','工作表']
    其中：时间为标准化HH:MM:SS；窗口为'AM'/'NOON'/None（无效时间不产出记录）。
    """
    records: List[Tuple[str, int, str, str, str, str]] = []

    for f in file_paths:
        try:
            # 按扩展名选择引擎，增强对.xls的兼容；.xlsx显式使用openpyxl
            lower = f.lower()
            engine = None
            if lower.endswith(".xls"):
                engine = "xlrd"
            elif lower.endswith(".xlsx") or lower.endswith(".xlsm"):
                engine = "openpyxl"
            xls = pd.ExcelFile(f, engine=engine)
        except Exception:
            continue

        for sheet_name in xls.sheet_names:
            try:
                df = pd.read_excel(f, sheet_name=sheet_name, header=None, dtype=object, engine=engine)
            except Exception:
                continue

            # 自动探测：若start_row为None，则从0开始全表扫描
            r = 0 if start_row is None else max(0, int(start_row))
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

def aggregate_daily(
    df_long: pd.DataFrame,
    year: int,
    month: int,
    rest_days: List[int],
    names_all: Optional[List[str]] = None,
) -> Tuple[pd.DataFrame, pd.DataFrame, pd.DataFrame]:
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
    # 汇总姓名集合：包含所有出现过的姓名 + 传入的全量姓名（即使无打卡也纳入）
    names_set = set(df_long["姓名"].unique()) if not df_long.empty else set()
    if names_all:
        names_set.update([n for n in names_all if n])

    results_detail: List[Dict] = []
    for name in sorted(names_set):
        sub = df_long[df_long["姓名"] == name] if not df_long.empty else pd.DataFrame(columns=df_long.columns if not df_long.empty else [])
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


def parse_all_names(file_paths: List[str], start_row: Optional[int] = None) -> List[str]:
    """扫描所有文件/工作表，收集出现的姓名（即使没有任何时刻）。"""
    names: List[str] = []
    for f in file_paths:
        try:
            lower = f.lower()
            engine = None
            if lower.endswith(".xls"):
                engine = "xlrd"
            elif lower.endswith(".xlsx") or lower.endswith(".xlsm"):
                engine = "openpyxl"
            xls = pd.ExcelFile(f, engine=engine)
        except Exception:
            continue
        for sheet_name in xls.sheet_names:
            try:
                df = pd.read_excel(f, sheet_name=sheet_name, header=None, dtype=object, engine=engine)
            except Exception:
                continue
            r = 0 if start_row is None else max(0, int(start_row))
            while r < df.shape[0]:
                res, next_r = _parse_person_block(df, r)
                if res is None:
                    r += 1
                    continue
                name = res.get("name")
                if name:
                    names.append(name)
                r = next_r
    # 去重保持顺序
    seen = set()
    uniq = []
    for n in names:
        if n not in seen:
            seen.add(n)
            uniq.append(n)
    return uniq
