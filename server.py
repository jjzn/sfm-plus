from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse
from datetime import datetime
import json
import info

PORT = 8420

class TrainServer(BaseHTTPRequestHandler):
    cache = {}

    def do_GET(self):
        code_str = self.path.split('/')[-1]
        code = int(code_str)

        # ensure code is valid
        assert list(info.codes.values()).count(code) == 1

        now = datetime.now()
        t = now.hour * 60 + now.minute

        if code in self.cache and self.cache[code][0] >= t:
            res = self.cache[code][1]
        else:
            res = json.dumps(info.retrieve_info(code))
            self.cache[code] = (t, res)

        self.send_response(502 if res == 'false' else 200)
        self.send_header('Content-Type', 'text/json')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()

        self.wfile.write(bytes(res + '\n', 'utf-8'))

server = HTTPServer(('', PORT), TrainServer)
print(f'serving on port {PORT}')
server.serve_forever()
