#include "listnode.h"

#include <sstream>
#include <vector>

ListNode* listNodeFrom(std::vector<int> input) {
    ListNode res(0);
    ListNode *node = &res;

    for (int n : input) { node->next = new ListNode(n); node = node->next; }

    return res.next;
}

bool operator==(const ListNode& lhs, const ListNode& rhs) {
    return (lhs.val == rhs.val)
        && !((lhs.next && !rhs.next) || (!lhs.next && rhs.next))
        && (!lhs.next || (*(lhs.next) == *(rhs.next)));
}

bool operator!=(const ListNode& lhs, const ListNode& rhs) {
    return !(lhs == rhs);
}

std::ostream& operator<<(std::ostream& os, const ListNode& value) {
    const ListNode *r = &value;

    os << "[";
    while (r) { os << r->val << ","; r = r->next; }
    os << "\b]";

    return os;
}

void destroy(ListNode* ln) {
    if (ln) { destroy(ln->next); delete ln; }
}
