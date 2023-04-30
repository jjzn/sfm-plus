from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse
from datetime import datetime
import requests
import json
import info
import io

PORT = 8420

class TrainServer(BaseHTTPRequestHandler):
    cache = {}
    session = requests.Session()

    def do_GET(self):
        code_str = self.path.split('/')[-1]
        code = int(code_str)

        # ensure code is valid
        if list(info.codes.values()).count(code) != 1:
            self.send_error(400)
            return

        now = datetime.now()
        t = now.hour * 60 + now.minute

        if code in self.cache and self.cache[code][0] >= t:
            table = self.cache[code][1]
        else:
            res = self.session.get(f'https://info.trensfm.com/sapi/ivi_imagen?ubicacion={code}')
            if not res:
                self.send_error(502)
                return

            im = io.BytesIO(res.content)
            table = info.retrieve_info(im)
            self.cache[code] = (t, table)

        self.send_response(200)
        self.send_header('Content-Type', 'text/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()

        codes_vals = list(info.codes.values())
        codes_keys = list(info.codes.keys())

        mesg = json.dumps({
            'station': codes_keys[codes_vals.index(code)],
            'table': table
        })
        self.wfile.write(bytes(mesg + '\n', 'utf-8'))

server = HTTPServer(('', PORT), TrainServer)
print(f'serving on port {PORT}')
server.serve_forever()
