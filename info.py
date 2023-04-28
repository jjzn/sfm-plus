import subprocess
import split
import json
import sys
import re
import io
import requests

codes = {
  "Intermodal": 1,
  "Jacint Verdaguer": 2,
  "Son Costa": 3,
  "Son Fuster Vell": 54,
  "Son Castelló": 55,
  "Gran Via Asima": 56,
  "Camí dels Reis": 57,
  "Son Sardina": 58,
  "UIB": 51,
  "Son Fuster": 4,
  "Son Cladera": 30,
  "Verge de Lluc": 6,
  "Pont d'Inca": 7,
  "Pont d'Inca Nou": 8,
  "Polígon de Marratxí": 9,
  "Marratxí": 10,
  "Es Caülls": 11,
  "Santa Maria": 12,
  "Consell - Alaró": 13,
  "Binissalem": 14,
  "Lloseta": 15,
  "Inca": 16,
  "Enllaç": 17,
  "Llubí": 18,
  "Muro": 19,
  "Sa Pobla": 20,
  "Sineu": 21,
  "Petra": 23,
  "Manacor": 24
}

station_map = {
    'uib': 'UIB',
    'sapobla': 'Sa Pobla'
}

_session = requests.Session()

def retrieve_info(code):
    res = _session.get(f'https://info.trensfm.com/sapi/ivi_imagen?ubicacion={code}')
    if not res:
        return False

    im = io.BytesIO(res.content)

    vals = []

    for i in range(7):
        if not split.split(i, im):
            break

        name = subprocess.run(['tesseract', '--dpi', '300', '--psm', '6', 'out/name.png', '-'], capture_output=True).stdout
        name = name.decode().strip().lower().replace(' ', '')

        rest = subprocess.run(['tesseract', '--dpi', '300', '--psm', '6', 'out/rest.png', '-'], capture_output=True).stdout
        rest = rest.decode().strip()

        rest_match = re.search('(\d\d?:\d\d) ?[^ \d]* ?(\d+)$', rest)
        time = rest_match.group(1)
        track = int(rest_match.group(2))

        dir = re.search('inca|manacor|sapobla|palma|marratxí|uib', name)
        if dir:
            title = station_map.get(dir.group(0), dir.group(0).title())
            vals.append({ 'title': title, 'time': time, 'track': track })

    code_idx = list(codes.values()).index(code)
    return {
        'station': list(codes.keys())[code_idx],
        'table': vals
    }

if __name__ == '__main__':
    vals = retrieve_info(codes[sys.argv[1]])
    print(json.dumps(vals))
