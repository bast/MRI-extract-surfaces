from collections import defaultdict, deque
import argparse


def read_mesh(datafile):
    points = []
    triangles = []

    with open(datafile, "r") as f:
        # how many points?
        n = int(f.readline())
        for _ in range(n):
            _x, _y, _z = map(float, f.readline().split())
            points.append((_x, _y, _z))
        # how many triangles?
        n = int(f.readline())
        for _ in range(n):
            _i, _j, _k = map(int, f.readline().split())
            triangles.append((_i, _j, _k))

    return points, triangles


def write_mesh(datafile, points, triangles):
    with open(datafile, "w") as f:
        f.write(f"{len(points)}\n")
        for x, y, z in points:
            f.write(f"{x} {y} {z}\n")
        f.write(f"{len(triangles)}\n")
        for i, j, k in triangles:
            f.write(f"{i} {j} {k}\n")


def ordered(a, b):
    return (a, b) if a < b else (b, a)


def triangle_neighbors(triangles):
    edge_to_triangles = defaultdict(list)
    for a, b, c in triangles:
        edge_to_triangles[ordered(a, b)].append((a, b, c))
        edge_to_triangles[ordered(b, c)].append((a, b, c))
        edge_to_triangles[ordered(c, a)].append((a, b, c))

    neighbors = defaultdict(list)

    for edge, triangles in edge_to_triangles.items():
        if len(triangles) == 2:
            a, b = edge
            t1, t2 = tuple(triangles)
            neighbors[t1].append(t2)
            neighbors[t2].append(t1)

    return neighbors


def visit_all_triangles(triangles, start_triangle):
    """
    Start with start_triangle
    Then visit all its neighbors before going anywhere else
    Then visit all neighbors of visited triangles
    And so on ...
    """
    visited = set()
    visit_list = []
    neighbors = triangle_neighbors(triangles)

    # to_visit is a deque
    to_visit = deque([start_triangle])
    while to_visit:
        triangle = to_visit.popleft()
        if triangle in visited:
            continue
        visited.add(triangle)
        visit_list.append(triangle)
        to_visit.extend(neighbors[triangle])

    # FIXME: for the moment we drop unreachable triangles
    # assert len(triangles) == len(visit_list)

    return visit_list


def _orient_triangles(triangles, drop_bad_triangles=False):
    start_triangle = triangles[0]
    visit_list = visit_all_triangles(triangles, start_triangle)

    half_edges = set()

    oriented_triangles = []
    for a, b, c in visit_list:
        if (a, b) in half_edges or (b, c) in half_edges or (c, a) in half_edges:
            if not drop_bad_triangles:
                oriented_triangles.append((c, b, a))
                half_edges.add((c, b))
                half_edges.add((b, a))
                half_edges.add((a, c))
        else:
            oriented_triangles.append((a, b, c))
            half_edges.add((a, b))
            half_edges.add((b, c))
            half_edges.add((c, a))

    return oriented_triangles


def _find_boundary_indices(triangles):
    """
    Find the indices of the boundary vertices
    """

    edge_to_vertex = defaultdict(list)
    for a, b, c in triangles:
        edge_to_vertex[ordered(a, b)].append(c)
        edge_to_vertex[ordered(b, c)].append(a)
        edge_to_vertex[ordered(c, a)].append(b)

    boundary_indices = defaultdict(int)
    for (a, b), vs in edge_to_vertex.items():
        if len(vs) == 1:
            boundary_indices[a] += 1
            boundary_indices[b] += 1

    double_boundary_indices = {k for k, v in boundary_indices.items() if v > 2}

    new_triangles = []
    for a, b, c in triangles:
        if (
            a in double_boundary_indices
            or b in double_boundary_indices
            or c in double_boundary_indices
        ):
            continue
        new_triangles.append((a, b, c))

    return new_triangles


def add_edge(adj, u, v):
    adj[u].add(v)
    adj[v].add(u)


def dfs(node, adj, visited):
    stack = [node]
    while stack:
        vertex = stack.pop()
        if not visited[vertex]:
            visited[vertex] = True
            for neighbor in adj[vertex]:
                if not visited[neighbor]:
                    stack.append(neighbor)


def number_of_components(edges):
    if not edges:
        return 0

    # Creating an adjacency list
    adj = {}
    for u, v in edges:
        if u not in adj:
            adj[u] = set()
        if v not in adj:
            adj[v] = set()
        add_edge(adj, u, v)

    # Visiting all nodes and counting components
    visited = {key: False for key in adj}
    component_count = 0

    for node in adj:
        if not visited[node]:
            dfs(node, adj, visited)
            component_count += 1

    return component_count


def _find_hourglass_indices(triangles):
    indices = set()
    for a, b, c in triangles:
        indices.add(a)
        indices.add(b)
        indices.add(c)

    opposite_edges = defaultdict(set)
    for a, b, c in triangles:
        opposite_edges[a].add(ordered(b, c))
        opposite_edges[b].add(ordered(c, a))
        opposite_edges[c].add(ordered(a, b))

    bad_indices = []
    for index in indices:
        components = number_of_components(opposite_edges[index])
        if components > 1:
            bad_indices.append(index)

    new_triangles = []
    for a, b, c in triangles:
        if a in bad_indices or b in bad_indices or c in bad_indices:
            continue
        new_triangles.append((a, b, c))

    return new_triangles


def remove_unreferenced_indices(points, triangles):
    indices = set()
    for i, j, k in triangles:
        indices.add(i)
        indices.add(j)
        indices.add(k)

    new_points = []
    new_indices = {}
    j = 0
    for i in indices:
        new_points.append(points[i])
        new_indices[i] = j
        j += 1

    new_triangles = []
    for i, j, k in triangles:
        new_triangles.append((new_indices[i], new_indices[j], new_indices[k]))

    return new_points, new_triangles


def mesh_cleanup(input_file, output_file):

    points, triangles = read_mesh(input_file)

    triangles = _orient_triangles(triangles, drop_bad_triangles=True)
    triangles = _find_boundary_indices(triangles)
    triangles = _find_hourglass_indices(triangles)

    points, triangles = remove_unreferenced_indices(points, triangles)

    write_mesh(output_file, points, triangles)


def parse_args():
    parser = argparse.ArgumentParser(description="Mesh cleanup")
    parser.add_argument("--input-file", type=str, help="Input file")
    parser.add_argument("--output-file", type=str, help="Output file")
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    mesh_cleanup(args.input_file, args.output_file)
