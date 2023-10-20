import json
from typing import Union

from flask import Flask, json as fson
from flask_cors import CORS

from dto.DTO import DTO
from dto.NumpyEncoder import NumpyJSONEncoder

app = Flask(__name__)
cors = CORS(app, resources={r"/*": {"origins": "*"}})


def to_json_response(data: Union[dict, DTO]):
    return fson.jsonify(json.loads(json.dumps(data, cls=NumpyJSONEncoder)))
