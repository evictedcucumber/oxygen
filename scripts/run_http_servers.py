#!/usr/bin/env python

from http.server import HTTPServer, SimpleHTTPRequestHandler
from threading import Thread
from os.path import abspath, dirname, join, isdir
from functools import partial
import signal
from typing import AnyStr

class DocHTTPRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, directory=None, **kwargs):
        super().__init__(*args, directory=directory, **kwargs)

    def do_GET(self):
        if self.path == "/":
            self.path = "/o2c"
        return super().do_GET()

def start_server(server: HTTPServer, dir: AnyStr) -> None:
    try:
        print(f"Started server on {server.server_address[0]}:{server.server_port} for {dir}")
        server.serve_forever()
    except KeyboardInterrupt:
        pass

if __name__ == "__main__":
    base_dir = abspath(dirname(__file__))
    servers: list[HTTPServer] = []
    threads: list[Thread] = []

    doc_dir = abspath(join(base_dir, "../target/doc"))
    if isdir(doc_dir):
        doc_server = HTTPServer(("", 8000), partial(DocHTTPRequestHandler,
                                                    directory=doc_dir))
        servers.append(doc_server)

        doc_thread = Thread(target=start_server, args=(doc_server,doc_dir))
        doc_thread.start()
        threads.append(doc_thread)
    else:
        print(f"{doc_dir} does not exist, so no server was started")

    cov_dir = abspath(join(base_dir, "../target/debug/coverage"))
    if isdir(cov_dir):
        cov_server = HTTPServer(("", 8001), partial(SimpleHTTPRequestHandler,
                                                    directory=cov_dir))
        servers.append(cov_server)

        cov_thread = Thread(target=start_server, args=(cov_server,cov_dir))
        cov_thread.start()
        threads.append(cov_thread)
    else:
        print(f"{cov_dir} does not exist, so no server was started")


    if len(servers) == 0 and len(threads) ==0:
        print("No servers created, exiting...")
        exit(0)

    try:
        signal.pause()
    except KeyboardInterrupt:
        print("Shutting down servers...")
        for server in servers:
            server.shutdown()
    finally:
        for thread in threads:
            thread.join()
        exit(0)
