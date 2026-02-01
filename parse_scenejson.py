import json

json_string: dict

with open("./test/output/scene.json", "r") as f:
    json_string = json.load(f)

objects = json_string.get("objects")
object_keys = set()

if not objects:
    quit()

for object in objects:
    if isinstance(object, dict):
        object_keys.update([(i, type(object.get(i))) for i in object.keys()])

for i in sorted(object_keys):
    print(i)
