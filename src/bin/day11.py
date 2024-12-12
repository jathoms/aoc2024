
from __future__ import annotations
import sys
import itertools
from dataclasses import dataclass

input_file_path = sys.argv[1]

stones = []

sys.setrecursionlimit(999999999)


@dataclass
class StoneNode:
    n: int
    children: list

    def populate_hashmap(self, hashmap: dict, depth=0, limit=10000):
        if depth > limit:
            return None
        if self.n in hashmap:
            self.children = hashmap[self.n]
            return self

        naive_children = iterate_stone(self.n)

        for child in naive_children:
            if child in hashmap:
                self.children.append(hashmap[child])
            else:
                child_node = StoneNode(child, [])
                self.children.append(
                    child_node.populate_hashmap(hashmap, depth=depth + 1)
                )

        hashmap[self.n] = self
        # print(f"inserted {self.n}, len->{len(hashmap)}")
        return self

    def f(
        self,
        hash,
    ):
        pass

@dataclass
class StoneWithParent:
    parent: StoneWithParent 
    n: int
    
    def __hash__(self):
        return hash(self.n)

    def do_iteration(self) -> list[StoneWithParent]:
        return [StoneWithParent(self, n) for n in iterate_stone(self.n)]


def iterate_stone(s: int) -> list:
    if s == 0:
        return [1]
    s_str = str(s)
    if len(s_str) % 2 == 0:
        # print("s_str:",s_str)
        # left = int(s_str[:len(s_str)//2])
        # right = int(s_str[len(s_str)//2:])
        # print("left",left)
        # print("right",right)
        return [int(s_str[: len(s_str) // 2]), int(s_str[len(s_str) // 2 :])]

    return [s * 2024]


with open(input_file_path) as f:
    input_string = f.read()
    print("input string:", input_string)


stones = [int(s) for s in input_string.split(" ")]

print(stones)

stone_nodes = [StoneNode(stone, []) for stone in stones]

hashmap = dict()

for node in stone_nodes:
    print("node n:", node.n)
    node.populate_hashmap(hashmap)

# node = stone_nodes[0]
# print([c.n for c in node.children])
print(len(hashmap))

for i in range(25):
    stones = list(
        itertools.chain.from_iterable([iterate_stone(stone) for stone in stones])
    )
    # print(stones)
print("part1: ", len(stones))

result = 0
stones_prnt = [StoneWithParent(None, stone) for stone in stones]

for depth in range(75):
    print(depth)
    next_stones = []
    for stone in stones_prnt:
        if (stone, depth) in hashmap:
            result += hashmap[(stone, depth)]
        else:
            current_depth = depth
            next_stones.extend(stone.do_iteration())
            while stone.parent:
                hashmap.setdefault((stone, current_depth), 0)
                hashmap[(stone, current_depth)] += 1
                stone = stone.parent
                current_depth -= 1
    stones_prnt = next_stones

print(result)

# for i in range(75):
#     print(f"iter {i}: {len(stone_nodes)}")
#     stone_nodes = list(
#         itertools.chain.from_iterable(
#             [[] if node is None else node.children for node in stone_nodes]
#         )
#     )

print("part2: ", len(stone_nodes))
