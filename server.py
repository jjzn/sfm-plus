from http.server import BaseHTTPRequestHandler, HTTPServer
from datetime import datetime
import argparse
import json
import io
import requests
import info

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
            cached_time, table = self.cache[code]
        else:
            res = self.session.get(f'https://info.trensfm.com/sapi/ivi_imagen?ubicacion={code}')
            if not res:
                self.send_error(502)
                return

            im = io.BytesIO(res.content)
            table = info.retrieve_info(im)
            cached_time = t
            self.cache[code] = (t, table)

        self.send_response(200)
        self.send_header('Content-Type', 'text/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()

        codes_vals = list(info.codes.values())
        codes_keys = list(info.codes.keys())

        mesg = json.dumps({
            'station': codes_keys[codes_vals.index(code)],
            'updated': cached_time,
            'table': table
        })
        self.wfile.write(bytes(mesg + '\n', 'utf-8'))

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-p', '--port', type=int, default=8420)
    args = parser.parse_args()

    server = HTTPServer(('', args.port), TrainServer)
    print(f'serving on port {args.port}')
    server.serve_forever()
