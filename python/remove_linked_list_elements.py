#  Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.

class ListNode:
   def __init__(self, val=0, next=None):
       self.val = val
       self.next = next


def removeElements(head: ListNode, val: int) -> ListNode:
    ptr = ListNode(0)
    ptr.next = head

    prev, curr = ptr, head
    while curr:
        if curr.val == val:
            prev.next = curr.next
        else:
            prev = curr

        curr = curr.next

    return ptr.next
