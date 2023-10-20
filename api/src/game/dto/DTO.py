from abc import ABC


class DTO(ABC, dict):
    pass
    # def to_json(self) -> str:
    #     def dumper(obj):
    #         try:
    #             return obj.to_json()
    #         except:
    #             return obj.__dict__
    #
    #     return json.dumps(self, default=dumper,
    #                       sort_keys=True, indent=4, cls=NumpyJSONEncoder)

