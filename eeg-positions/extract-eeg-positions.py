import sys


input_file = sys.argv[-1]


d = {
    "Cz": "vertex",
    "Nz": "nasion",
    "Iz": "inion",
    "LPA": "left tragus",
    "RPA": "right tragus",
}


print("position,x,y,z")
with open(input_file, "r") as f:
    for line in f.read().splitlines():
        _, x, y, z, tag = line.split(",")
        if tag in d:
            print(f"{d[tag]},{x},{y},{z}")
