from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        return f"ListNode({self.val}, {self.next})"

    def __eq__(self, other):
        if isinstance(other, ListNode):
            return self.val == other.val and (
                other.next is None if self.next is None else self.next == other.next
            )
        else:
            return False


def listNodeFrom(input: list[int]) -> Optional[ListNode]:
    """Constructs a `ListNode` from `input`"""
    res = ListNode(0)
    curr = res

    for n in input:
        curr.next = ListNode(n)
        curr = curr.next

    return res.next
