from flask import Flask
from components import data_disk

app = Flask(__name__)

@app.route('/api_disk', methods=['GET'])
def disk_r():
    return data_disk()

if __name__ == '__main__':
    app.run(use_reloader=True, port=9000, debug=True)