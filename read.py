from simnibs import read_msh
from collections import defaultdict
import os
import argparse


def remove_unreferenced_indices(points, vertices):
    indices = set()
    for i, j, k in vertices:
        indices.add(i)
        indices.add(j)
        indices.add(k)

    new_points = []
    new_indices = {}
    j = 0
    for i in indices:
        new_points.append(points[i + 1])
        new_indices[i] = j
        j += 1

    new_vertices = []
    for i, j, k in vertices:
        new_vertices.append((new_indices[i], new_indices[j], new_indices[k]))

    return new_points, new_vertices


def write_data(vertices, faces, file_name):
    vertices, faces = remove_unreferenced_indices(vertices, faces)
    with open(file_name, "w") as file:
        num_vertices = max([i for face in faces for i in face]) + 1
        num_faces = len(faces)
        file.write(f"{num_vertices}\n")
        for i in range(num_vertices):
            x, y, z = vertices[i]
            file.write(f"{x} {y} {z}\n")
        file.write(f"{num_faces}\n")
        for i, j, k in faces:
            file.write(f"{i} {j} {k}\n")


def main(input_mesh, output_path):
    mesh = read_msh(input_mesh)
    os.makedirs(output_path, exist_ok=True)

    faces = defaultdict(list)
    faces["all"] = []
    for e, (i, j, k, l) in enumerate(mesh.elm.node_number_list):
        if l == -1:
            tag = str(mesh.elm.tag1[e])
            faces[tag].append((i - 1, j - 1, k - 1))
            faces["all"].append((i - 1, j - 1, k - 1))

    for tag in faces:
        output_file = os.path.join(output_path, f"{tag}.txt")
        write_data(mesh.nodes, faces[tag], output_file)


def parse():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input-mesh", type=str)
    parser.add_argument("--output-path", type=str)
    return parser.parse_args()


if __name__ == "__main__":
    args = parse()
    main(args.input_mesh, args.output_path)
