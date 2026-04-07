# 旧实现缺陷与歧义清单

## 1. AM/NOON 窗口口径冲突

### 冲突位置

1. `README.md`
   - `AM <= 09:04:59`
   - `NOON 11:00:00~14:04:59`
2. `app/processor.py`
   - `AM_CUTOFF = 09:11:59`
   - `NOON_END = 14:11:59`
3. `selfcheck.py`注释
   - 把`14:05`写成“越界无效”

### 实际问题

旧系统文档、注释、代码三者不一致，导致边界时间无从审计。

### 新系统结论

以旧代码真实行为为默认预设，并把窗口改为规则配置。

## 2. `_parse_person_block`使用脆弱日期门槛

### 位置

- `app/processor.py::_parse_person_block`

### 旧逻辑

姓名行后的候选日期行，只有在识别到`>=5`个日号时才视为有效日期行。

### 风险

1. 短样本表会漏掉整个人员块
2. 异常表、补录表、分段表会漏解析
3. 直接影响姓名识别、打卡识别和后续汇总

### 新系统结论

改为评分式日期行探测，不再依赖`>=5`硬门槛。

## 3. `selfcheck`注释与真实行为不一致

### 位置

- `selfcheck.py`
- `app/processor.py::_which_window`

### 旧问题

`selfcheck.py`写明：

- `14:05`是越界，不计入有效`NOON`

但`app/processor.py`实际实现：

- `NOON_END = 14:11:59`
- 所以`14:05`会被计入有效`NOON`

### 新系统结论

冻结为显式规则：

- 默认预设下`14:05`计入有效`NOON`

## 4. `parse_all_names`复用脆弱解析门槛

### 位置

- `app/processor.py::parse_all_names`

### 旧问题

`parse_all_names`直接复用了`_parse_person_block`，也会受到
`>=5`日期列门槛影响。

### 风险

1. 识别姓名集合不完整
2. `0`打卡人员可能直接从汇总链路消失

### 新系统结论

姓名识别与打卡 token 识别要解耦：

1. 姓名块识别成功即可纳入姓名全集
2. 即使后续没有有效打卡，也必须进入汇总与通报名单

## 5. 旧系统对异常文件的容错粒度偏粗

### 位置

- `app/processor.py::parse_excel_files`

### 旧问题

旧系统对文件打开和 sheet 读取做了基础`try/except`，但没有形成稳定的
结构化错误对象，也没有对块级错误、单元格定位信息做统一输出。

### 新系统结论

1. 文件级错误不拖垮整批任务
2. 错误对象需保留：
   - 文件名
   - 工作表名
   - 行号
   - 列号
   - 错误类别

## 6. 旧系统路径与日志泄漏面较大

### 位置

- `main_dakaprocess.py`
- `app/processor.py`

### 旧问题

1. 明细记录里保留完整源文件路径
2. 日志直接输出文件完整绝对路径

### 风险

桌面端场景下，这会直接增加个人信息和本地目录信息泄漏面。

### 新系统结论

默认日志和前端 DTO 只暴露脱敏后的路径信息。

## 7. 旧系统导出能力不足

### 位置

- `app/exporter.py`

### 旧问题

旧系统只支持：

1. 汇总`xlsx`
2. 汇总`csv`
3. 可选明细 sheet
4. 可选需要打卡日 sheet

缺少：

1. 通报名单导出
2. 路径校验与文件名规范化
3. 结构化导出错误

## 8. 旧系统架构耦合

### 位置

- `main_dakaprocess.py`
- `app/processor.py`
- `app/exporter.py`
- `app/ui_main.py`

### 旧问题

1. Qt 界面、线程、解析、聚合、导出耦合在一起
2. 业务规则以常量形式散落
3. DataFrame 是事实上的内部总线

### 新系统结论

必须拆分为：

1. Rust domain
2. Rust application
3. Rust infrastructure
4. Rust commands
5. React pages/components/store/types
