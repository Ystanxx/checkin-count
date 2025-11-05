import pandas as pd
from app.processor import _parse_person_block

path = r".\\Oct\\10月员工刷卡记录表.xlsx"
df = pd.read_excel(path, sheet_name="员工刷卡记录表", header=None, dtype=object, engine="openpyxl")
res, next_r = _parse_person_block(df, 4)
print('res is None?', res is None, 'next_r=', next_r)
print('res keys:', list(res.keys()) if res else None)
if res:
    print('name:', res['name'])
    dtt = res['day_to_tokens']
    print('days keys:', sorted(dtt.keys())[:10])
    total_tokens = sum(len(v) for v in dtt.values())
    print('total tokens:', total_tokens)
    # show first few tokens by day
    for k in sorted(dtt.keys())[:5]:
        print(k, '->', dtt[k][:10])
