import re
from pathlib import Path
import pyperclip

debug = True

APP = Path(__file__).parent
DIST = APP / 'dist'
SRC = APP / 'src'

files = SRC.glob('*')
template = {}
for file in files:
    template[f"{{{file.name}}}"] = file.read_text(encoding='utf-8')

text: str = template['{main.rs}']
del template['{main.rs}']


def replace(text: str, template: dict):
    res = ""
    for line in text.splitlines():
        if "python:del" in line:
            continue
        elif "python:replace" in line:
            text = line.split("python:replace")[1]
            for key in template.keys():
                text = text.replace(key, template[key])
            res += text + "\n"
            continue
        else:
            res += line + "\n"
    return res


def del_comment(text: str):
    res = ""
    for line in text.splitlines():
        if "\\\\" in line:
            res += line.split("\\\\")[0] + "\n"
        else:
            res += line + "\n"
    return res


for key in template.keys():
    template[key] = replace(template[key], template)

text = replace(text, template)
if not debug:
    text = del_comment(text)
    text = text.replace("\n", "")
    text = re.sub(r' +', ' ', text)

(DIST / 'main.rs').write_text(text, encoding='utf-8')

pyperclip.copy(text)