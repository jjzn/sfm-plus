from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse
import json
import info

PORT = 8420

class TrainServer(BaseHTTPRequestHandler):
    def do_GET(self):
        code_str = self.path.split('/')[-1]
        code = int(code_str)

        # ensure code is valid
        assert list(info.codes.values()).count(code) == 1

        res = json.dumps(info.retrieve_info(code))

        self.send_response(200)
        self.send_header('Content-Type', 'text/json')
        self.end_headers()

        self.wfile.write(bytes(res + '\n', 'utf-8'))

server = HTTPServer(('', PORT), TrainServer)
print(f'serving on port {PORT}')
server.serve_forever()
