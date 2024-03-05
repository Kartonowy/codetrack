const fs = require("fs")

class Color {
    constructor(r, g, b) {
        this.r = r
        this.g = g
        this.b = b
    }
}
class Language {
    constructor(ext, name, col) {
        this.ext = ext
        this.name = name
        this.col = col
    }
}

function hexToRgb(hex) {
  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return new Color(parseInt(result[1], 16), parseInt(result[2], 16), parseInt(result[3], 16)) }

const xd = () => {
    let read_data = JSON.parse(fs.readFileSync(__dirname + "/../items.json",'utf-8'))

    let golang = new Language("php", "PHP", hexToRgb("#4F5D95"))
    console.log(read_data)

    read_data.push(golang);

    fs.writeFile(__dirname + "/../items.json", JSON.stringify(read_data, null, 4), (err) => {
        if (err) throw err;
    })

}

xd()
