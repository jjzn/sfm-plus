import subprocess
import json
import sys
import re
import io
import requests
import split

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
    'inca': 'Inca',
    'manacor': 'Manacor',
    'sapobla': 'Sa Pobla',
    '2apobla': 'Sa Pobla',
    'safobla': 'Sa Pobla',
    '2afobla': 'Sa Pobla',
    'palma': 'Palma',
    'ralma': 'Palma'
}

def retrieve_info(im):
    vals = []

    for i in range(7):
        if not split.split(i, im):
            continue

        name = subprocess.run(['tesseract', '--dpi', '300', '--psm', '10', 'out/name.png', '-'], capture_output=True).stdout
        name = name.decode().strip().lower().replace(' ', '')

        rest = subprocess.run(['tesseract', '--dpi', '300', '--psm', '11', 'out/rest.png', '-'], capture_output=True).stdout
        rest = rest.decode().strip()

        rest_match = re.search(r'(?ms)(\d\d?:\d\d).+(\d+)$', rest)
        if not rest_match:
            continue

        time = rest_match.group(1)
        track = int(rest_match.group(2))

        dir = re.search('|'.join(station_map.keys()), name)
        if not dir:
            continue

        title = station_map.get(dir.group(0), dir.group(0).title())
        vals.append({ 'title': title, 'time': time, 'track': track })

    return vals

if __name__ == '__main__':
    res = requests.get(f'https://info.trensfm.com/sapi/ivi_imagen?ubicacion={codes[sys.argv[1]]}')
    if not res:
        raise Exception(f'SFM image request returned {res.status_code}')

    im = io.BytesIO(res.content)
    vals = retrieve_info(im)

    print(json.dumps({
        'station': sys.argv[1],
        'table': vals
    }))
