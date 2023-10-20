import json
from typing import Union

from flask import Flask, json as fson
from flask_cors import CORS

from game.dto.DTO import DTO
from game.dto.NumpyEncoder import NumpyJSONEncoder

app = Flask(__name__)
cors = CORS(app, resources={r"/*": {"origins": "*"}})


def to_json_response(data: Union[dict, DTO]):
    return fson.jsonify(json.loads(json.dumps(data, cls=NumpyJSONEncoder)))
