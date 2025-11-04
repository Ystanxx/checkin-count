from __future__ import annotations

# 中文注释：Qt界面主窗口

from typing import List

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QAbstractItemView,
    QApplication,
    QCheckBox,
    QComboBox,
    QGridLayout,
    QGroupBox,
    QHBoxLayout,
    QLabel,
    QListWidget,
    QListWidgetItem,
    QMainWindow,
    QMessageBox,
    QPushButton,
    QFileDialog,
    QSpinBox,
    QStatusBar,
    QTextEdit,
    QVBoxLayout,
    QWidget,
)


class MainWindow(QMainWindow):
    """主窗口，包含文件选择、年月选择、休息日选择，以及导出控制。"""

    def __init__(self) -> None:
        super().__init__()
        self.setWindowTitle("打卡数据处理 - Qt6")
        self.resize(1100, 700)

        central = QWidget(self)
        self.setCentralWidget(central)
        layout = QVBoxLayout(central)

        # 文件选择区域
        file_box = QGroupBox("文件选择（可多选）")
        file_layout = QHBoxLayout(file_box)
        self.btn_add_files = QPushButton("添加Excel文件")
        self.btn_clear_files = QPushButton("清空文件")
        self.list_files = QListWidget()
        self.list_files.setSelectionMode(QAbstractItemView.ExtendedSelection)
        file_layout.addWidget(self.btn_add_files)
        file_layout.addWidget(self.btn_clear_files)
        file_layout.addWidget(self.list_files, 1)
        layout.addWidget(file_box)

        # 年月与休息日区域
        ym_box = QGroupBox("年月与休息日")
        ym_layout = QVBoxLayout(ym_box)
        ym_top = QHBoxLayout()
        ym_layout.addLayout(ym_top)

        self.year_spin = QSpinBox()
        self.year_spin.setRange(1970, 2100)
        self.year_spin.setValue(QApplication.instance().arguments()[0:0] and 2024 or 2024)

        self.month_spin = QSpinBox()
        self.month_spin.setRange(1, 12)
        self.month_spin.setValue(1)

        ym_top.addWidget(QLabel("年份"))
        ym_top.addWidget(self.year_spin)
        ym_top.addWidget(QLabel("月份"))
        ym_top.addWidget(self.month_spin)
        ym_top.addStretch(1)

        grid_box = QGroupBox("选择休息日（当月1-31日）")
        grid = QGridLayout(grid_box)
        self.day_checks: List[QCheckBox] = []
        for d in range(1, 32):
            cb = QCheckBox(str(d))
            self.day_checks.append(cb)
            r = (d - 1) // 8
            c = (d - 1) % 8
            grid.addWidget(cb, r, c)
        ym_layout.addWidget(grid_box)

        action_bar = QHBoxLayout()
        self.btn_select_weekends = QPushButton("勾选双休(简单)")
        self.btn_clear_rest = QPushButton("清空休息日")
        action_bar.addWidget(self.btn_select_weekends)
        action_bar.addWidget(self.btn_clear_rest)
        action_bar.addStretch(1)
        ym_layout.addLayout(action_bar)

        layout.addWidget(ym_box)

        # 操作与导出
        op_box = QGroupBox("解析与导出")
        op_layout = QHBoxLayout(op_box)
        self.btn_parse = QPushButton("读取并解析")
        self.chk_export_detail = QCheckBox("导出明细")
        self.chk_export_need_days = QCheckBox("导出需要打卡日页")
        self.btn_export_excel = QPushButton("导出汇总(xlsx)")
        self.btn_export_csv = QPushButton("导出汇总(csv UTF-8 BOM)")
        op_layout.addWidget(self.btn_parse)
        op_layout.addWidget(self.chk_export_detail)
        op_layout.addWidget(self.chk_export_need_days)
        op_layout.addStretch(1)
        op_layout.addWidget(self.btn_export_excel)
        op_layout.addWidget(self.btn_export_csv)
        layout.addWidget(op_box)

        # 日志区域
        self.log = QTextEdit()
        self.log.setReadOnly(True)
        layout.addWidget(self.log, 1)

        self.status = QStatusBar(self)
        self.setStatusBar(self.status)

    # ============ 便捷方法 ============
    def get_selected_files(self) -> List[str]:
        paths: List[str] = []
        for i in range(self.list_files.count()):
            item = self.list_files.item(i)
            paths.append(item.text())
        return paths

    def set_files(self, files: List[str]) -> None:
        self.list_files.clear()
        for f in files:
            it = QListWidgetItem(f)
            self.list_files.addItem(it)

