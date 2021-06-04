### 向量

```py
v = [1, 2, 3]

v[-2] # 2
len(v) # 3

v.append(5)
del v[0]

v = [1, 2, 3] + [4, 5] # [1, 2, 3, 4, 5]

for x in v:
    print(x)

if 3 in v:
    None
```

###### Other Slice

```py
v = [1, 2, 3]
v[1:] # [2, 3]
```

### dict

```py
for k, v in d.iteritems():

for k in d:
```

## 列出目录中的内容

```py
import os
import re

rx = re.compile(r'\.(rs|md)')

for x in os.listdir('.'):
    if rx.search(x)]:
        None

    if x.endswith('.js'):
        None

    if os.path.isfile(x):
        None

    if os.path.isdir(x):
        None

    if os.path.islink(x):
        None

# 循环遍历
for x in os.walk('.'):
    print(x)

for path, dnames, fnames in os.walk('.'):
    None

# more
# finding files
rx = re.compile(r'\.(js|md)')
r = []
for path, dnames, fnames in os.walk('.'):
    r.extend([os.path.join(path, x) for x in fnames if rx.search(x)])
print r

# find files larger than 10000 bytes
r = []
for path, dnames, fnames in os.walk('.'):
    r.extend([os.path.join(path, x) for x in fnames if os.path.getsize(os.path.join(path, x)) > 10000])
print r

# files that have been modified less than 8 hours ago?
r = []
now = int(time.time())
for path, dnames, fnames in os.walk('.'):
    r.extend([os.path.join(path, x) for x in fnames if (now - int(os.path.getmtime(os.path.join(path, x))))/(60*60) < 8])
print r
```

# 时间

## rfc3339

```py
import dateutil.parser

v = dateutil.parser.parse(d)
v.strftime("%Y-%m-%d %H:%M:%S")
```

## 当前时间

```py
import time
import datetime

######
d = time.strftime("%Y%m%d")

######
d = datetime.datetime.now()
w = d.weekday()
h = d.hour
s = d.strftime("%Y-%m-%d %H:%M:%S")

######
d += datetime.timedelta(hours=8) # 当前时间增加８小时
d += datetime.timedelta(days=8)
d += datetime.timedelta(seconds=8)
d += datetime.timedelta(microseconds=8)
d += datetime.timedelta(milliseconds=8)
d += datetime.timedelta(minutes=8)
d += datetime.timedelta(hours=8)
d += datetime.timedelta(weeks=8)
```

/usr/lib/python2.7/xml/etree/ElementTree.py

## pip

```bash
sudo apt-get remove --purge python-pip
sudo apt-get autoremove
sudo rm -f /usr/local/bin/pip
sudo easy_install pip==20.3.4
pip --version
```