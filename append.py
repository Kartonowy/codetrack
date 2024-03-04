import json

class Color:
    def __init__(self, red, green, blue):
        self.red = red
        self.green = green
        self.blue = blue
        

class Language:
    def __init__(self, extension, name, color):
        self.extension = extension
        self.name = name
        self.color = color
    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)

items = open("./items.json", "r+")

golang = Language("go", "golang", Color(69, 69, 69))

decoded = json.loads(items.read())

print(decoded)
