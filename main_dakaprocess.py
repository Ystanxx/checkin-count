"""
主程序入口（Qt6）。

中文注释：
 - 提供文件选择（系统资源管理器），
 - 年月与31天休息日选择，
 - 解析并导出汇总与明细。
"""

from __future__ import annotations

import os
from datetime import date, timedelta
from typing import List, Optional

from PySide6.QtCore import QThread, Signal, QObject
from PySide6.QtWidgets import QApplication, QFileDialog, QMessageBox

from app.ui_main import MainWindow
from app.processor import parse_excel_files, aggregate_daily, month_days, parse_all_names
from app.exporter import export_to_excel, export_summary_csv_bom


def _pick_files() -> List[str]:
    """打开文件对话框选择Excel文件。"""
    dlg = QFileDialog()
    dlg.setFileMode(QFileDialog.ExistingFiles)
    dlg.setNameFilters(["Excel 文件 (*.xlsx *.xlsm *.xls)", "所有文件 (*.*)"])
    if dlg.exec():
        return dlg.selectedFiles()
    return []


class ParseWorker(QThread):
    """后台解析线程，避免阻塞UI。"""
    progress_sig = Signal(str)
    finished_sig = Signal(object, object, object, object)  # df_long, df_detail, df_summary, df_need_days
    error_sig = Signal(str)

    def __init__(self, files: List[str], start_row: Optional[int], year: int, month: int, rest_days: List[int]):
        super().__init__()
        self.files = files
        self.start_row = start_0 = start_row
        self.year = year
        self.month = month
        self.rest_days = rest_days

    def run(self):
        try:
            # 定义回调函数，通过信号发送日志
            def log_callback(msg: str):
                self.progress_sig.emit(msg)

            self.progress_sig.emit(f"开始解析：年={self.year} 月={self.month} 数据起始行={self.start_row} 休息日={self.rest_days}")
            
            df_long = parse_excel_files(self.files, start_row=self.start_row, log_callback=log_callback)
            names_all = parse_all_names(self.files, start_row=self.start_row)
            
            if df_long is None or df_long.empty:
                if names_all:
                    df_detail, df_summary, df_need_days = aggregate_daily(df_long, self.year, self.month, self.rest_days, names_all=names_all)
                    self.progress_sig.emit(f"未解析到有效打卡，但识别到姓名{len(names_all)}人，已按0次生成汇总。")
                    self.finished_sig.emit(df_long, df_detail, df_summary, df_need_days)
                else:
                    self.error_sig.emit("未解析到有效记录，请检查源文件格式")
                return

            df_detail, df_summary, df_need_days = aggregate_daily(df_long, self.year, self.month, self.rest_days, names_all=names_all)
            self.progress_sig.emit(f"解析完成：记录数={len(df_long)} 人数={df_summary.shape[0]}")
            self.finished_sig.emit(df_long, df_detail, df_summary, df_need_days)

        except Exception as e:
            self.error_sig.emit(f"解析过程发生错误: {str(e)}")


def main() -> None:
    """主函数：构建界面并连接事件。"""
    app = QApplication([])
    w = MainWindow()

    # 绑定：添加文件
    def on_add_files():
        files = _pick_files()
        if not files:
            return
        exist = set(w.get_selected_files())
        for f in files:
            if f not in exist:
                exist.add(f)
        w.set_files(sorted(list(exist)))
        w.log.append("已添加文件：\n" + "\n".join(sorted(list(exist))))

    w.btn_add_files.clicked.connect(on_add_files)

    # 绑定：清空文件
    def on_clear_files():
        w.set_files([])
        w.log.append("已清空文件列表")

    w.btn_clear_files.clicked.connect(on_clear_files)

    # 绑定：快速勾选周六周日（简单按当月首日推算）
    def on_select_weekends():
        y = w.year_spin.value()
        m = w.month_spin.value()
        md = month_days(y, m)
        for i, cb in enumerate(w.day_checks, start=1):
            cb.setChecked(False)
        # 简单推算周末
        first = date(y, m, 1)
        for d in range(1, md + 1):
            dt = first + timedelta(days=d - 1)
            if dt.weekday() >= 5:  # 5/6=周六/周日
                w.day_checks[d - 1].setChecked(True)
        w.log.append("已勾选当月周末为休息日")

    w.btn_select_weekends.clicked.connect(on_select_weekends)

    # 绑定：清空休息日
    def on_clear_rest():
        for cb in w.day_checks:
            cb.setChecked(False)
        w.log.append("已清空休息日")

    w.btn_clear_rest.clicked.connect(on_clear_rest)

    # 共享解析结果缓存
    cache = {
        "df_long": None,
        "df_detail": None,
        "df_summary": None,
        "df_need_days": None,
    }
    
    # 保存 worker 引用防止被垃圾回收
    w.worker = None

    # 绑定：读取并解析
    def on_parse():
        files = w.get_selected_files()
        if not files:
            QMessageBox.warning(w, "提示", "请先添加Excel文件")
            return
        
        # 禁用按钮防止重复点击
        w.btn_parse.setEnabled(False)
        w.status.showMessage("解析中，请稍候...")
        
        y = w.year_spin.value()
        m = w.month_spin.value()
        # 自动探测：当值<=1时，传递None以全表扫描
        start_1 = w.start_row_spin.value()
        start_0 = None if start_1 <= 1 else max(0, start_1 - 1)
        rest_days: List[int] = [i + 1 for i, cb in enumerate(w.day_checks) if cb.isChecked()]

        # 创建并启动线程
        w.worker = ParseWorker(files, start_0, y, m, rest_days)
        
        def on_progress(msg):
            w.log.append(msg)
            
        def on_finished(df_long, df_detail, df_summary, df_need_days):
            cache["df_long"] = df_long
            cache["df_detail"] = df_detail
            cache["df_summary"] = df_summary
            cache["df_need_days"] = df_need_days
            w.status.showMessage("解析完成")
            w.btn_parse.setEnabled(True)
            
        def on_error(msg):
            QMessageBox.critical(w, "错误", msg)
            w.status.clearMessage()
            w.btn_parse.setEnabled(True)

        w.worker.progress_sig.connect(on_progress)
        w.worker.finished_sig.connect(on_finished)
        w.worker.error_sig.connect(on_error)
        w.worker.start()

    w.btn_parse.clicked.connect(on_parse)

    # 绑定：导出xlsx
    def on_export_excel():
        if cache["df_summary"] is None:
            QMessageBox.information(w, "提示", "请先解析数据")
            return
        y = w.year_spin.value()
        m = w.month_spin.value()
        out, _ = QFileDialog.getSaveFileName(w, "保存为xlsx", f"汇总_{y:04d}{m:02d}.xlsx", "Excel 文件 (*.xlsx)")
        if not out:
            return
        df_detail = cache["df_detail"] if w.chk_export_detail.isChecked() else None
        df_need = cache["df_need_days"] if w.chk_export_need_days.isChecked() else None
        export_to_excel(out, cache["df_summary"], df_detail=df_detail, df_need_days=df_need)
        w.log.append(f"已导出xlsx：{out}")

    w.btn_export_excel.clicked.connect(on_export_excel)

    # 绑定：导出CSV(BOM)
    def on_export_csv():
        if cache["df_summary"] is None:
            QMessageBox.information(w, "提示", "请先解析数据")
            return
        y = w.year_spin.value()
        m = w.month_spin.value()
        out, _ = QFileDialog.getSaveFileName(w, "保存为csv", f"汇总_{y:04d}{m:02d}.csv", "CSV 文件 (*.csv)")
        if not out:
            return
        export_summary_csv_bom(out, cache["df_summary"])
        w.log.append(f"已导出csv：{out}")

    w.btn_export_csv.clicked.connect(on_export_csv)

    w.show()
    app.exec()


if __name__ == "__main__":
    main()
