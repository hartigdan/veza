import asyncio
import websockets
import json
import psutil
from datetime import datetime


async def dummy_data(websocket, path):
    while True:
        items = [{
            'id': 'cpu_percent',
            'value': psutil.cpu_percent()
        }, {
            'id': 'cpu_freq_current',
            'value': psutil.cpu_freq().current
        }, {
            'id': 'disk_io_read_bytes',
            'value': psutil.disk_io_counters().read_bytes
        }, {
            'id': 'disk_io_write_bytes',
            'value': psutil.disk_io_counters().write_bytes
        }, {
            'id': 'net_io_bytes_sent',
            'value': psutil.net_io_counters().bytes_sent
        }, {
            'id': 'net_io_bytes_recv',
            'value': psutil.net_io_counters().bytes_recv
        }, {
            'id': 'datetime_now',
            'value': f'{datetime.now()}'
        }]
        await websocket.send(json.dumps(items))
        await asyncio.sleep(1)


async def main():
    async with websockets.serve(dummy_data, "localhost", 8765):
        await asyncio.Future()  # run forever


if __name__ == "__main__":
    asyncio.run(main())
