#ifndef LISTNODE_H
#define LISTNODE_H

#include <string>
#include <ostream>
#include <vector>

struct ListNode {
    int val;
    ListNode *next;

    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

ListNode* listNodeFrom(std::vector<int> input);
bool operator==(const ListNode& lhs, const ListNode& rhs);
bool operator!=(const ListNode& lhs, const ListNode& rhs);
std::ostream& operator<<(std::ostream& os, const ListNode& value);
void destroy(ListNode* ln);

#endif
