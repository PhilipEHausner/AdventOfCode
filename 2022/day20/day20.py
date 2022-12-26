from __future__ import annotations

import enum
from typing import Optional


class Direction(enum.Enum):
    BACKWARD = 1
    FORWARD = 2


class Node:
    value: int
    _tail: Optional[Node]
    _head: Optional[Node]

    def __init__(self, value: int, tail: Optional[Node] = None,
                 head: Optional[Node] = None):
        super(Node, self).__init__()

        self.value = value
        self._tail = tail
        self._head = head

    @classmethod
    def from_value(cls, value: int) -> Node:
        return cls(value, None, None)

    @property
    def head(self) -> Node:
        if self._head is None:
            raise ValueError("Broken Node.")
        return self._head

    @head.setter
    def head(self, head: Node) -> None:
        self._head = head

    @property
    def tail(self) -> Node:
        if self._tail is None:
            raise ValueError("Broken Node.")
        return self._tail

    @tail.setter
    def tail(self, tail: Node) -> None:
        self._tail = tail

    def move_backward(self) -> None:
        tail_tail = self.tail.tail
        tail = self.tail
        head = self.head

        tail_tail.head = self
        tail.tail = self
        tail.head = head
        head.tail = tail
        self.tail = tail_tail
        self.head = tail

    def move_forward(self) -> None:
        tail = self.tail
        head = self.head
        head_head = self.head.head

        tail.head = head
        head.tail = tail
        head.head = self
        head_head.tail = self
        self.tail = head
        self.head = head_head

    def __repr__(self) -> str:
        return f"Node(value: {self.value}, tail: {id(self.tail)}, head: {id(self.head)})"


class CycleDoubleLinkedList:
    nodes: list[Node]

    def __init__(self, values: list[int]):
        super(CycleDoubleLinkedList, self).__init__()

        self.nodes = [Node.from_value(value) for value in values]
        self._add_node_linkage(self.nodes)

    @staticmethod
    def _add_node_linkage(nodes: list[Node]) -> None:
        for i in range(1, len(nodes)):
            nodes[i - 1].head = nodes[i]
            nodes[i].tail = nodes[i - 1]
        nodes[0].tail = nodes[-1]
        nodes[-1].head = nodes[0]

    def mix(self) -> None:
        for node in self.nodes:
            self._move_node(node)

    def _move_node(self, node: Node) -> None:
        # Instead of deciding the direction by the sign, we could always go the shorter
        # way to move the node to the correct position, since the intermediate steps
        # actually do not matter.
        direction = Direction.FORWARD if node.value > 0 else Direction.BACKWARD
        for _ in range(abs(node.value) % (len(self.nodes) - 1)):
            if direction == Direction.FORWARD:
                node.move_forward()
            else:
                node.move_backward()

    def get_node_x(self, x: int) -> Node:
        result = next(node for node in self.nodes if node.value == 0)
        for _ in range(x):
            result = result.head
        return result

    def __str__(self) -> str:
        res = []
        node = self.get_node_x(0)
        for _ in range(len(self.nodes)):
            res.append(str(node.value))
            node = node.head
        return " -> ".join(res)


def solve1(values: list[int]) -> int:
    linkedlist = CycleDoubleLinkedList(values)
    linkedlist.mix()
    node_1000 = linkedlist.get_node_x(1000)
    node_2000 = linkedlist.get_node_x(2000)
    node_3000 = linkedlist.get_node_x(3000)
    return node_1000.value + node_2000.value + node_3000.value


def solve2(values: list[int]) -> int:
    decrypted_values = [value * 811589153 for value in values]
    linkedlist = CycleDoubleLinkedList(decrypted_values)
    for _ in range(10):
        linkedlist.mix()
    node_1000 = linkedlist.get_node_x(1000)
    node_2000 = linkedlist.get_node_x(2000)
    node_3000 = linkedlist.get_node_x(3000)
    return node_1000.value + node_2000.value + node_3000.value


def main(file: str) -> None:
    values = [int(line) for line in open(file, "r").readlines()]
    print(f"Solution part 1: {solve1(values)}")
    print(f"Solution part 2: {solve2(values)}")


if __name__ == "__main__":
    import timeit
    start = timeit.default_timer()
    main("./files/day20.txt")
    end = timeit.default_timer()
    print(f"{end-start} seconds")
