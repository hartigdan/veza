import json
import psutil
import time
from datetime import datetime


def dummy_data():
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
        print(json.dumps(items), flush=True)
        time.sleep(1)


def main():
    dummy_data()


if __name__ == "__main__":
    main()
