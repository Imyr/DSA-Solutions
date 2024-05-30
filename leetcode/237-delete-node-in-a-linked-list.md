# 237. Delete Node in a Linked List
- [Submission](https://leetcode.com/submissions/detail/1249951539/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| C | 6 ms | 6.4 MB |
```
void deleteNode(struct ListNode* node) {
    *node = *node->next; 
}
```
- [Submission](https://leetcode.com/submissions/detail/1112745519/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| C | 0 ms | 7.3 MB |
```
void deleteNode(struct ListNode* node) {
    node->val = node->next->val;
    node->next = node->next->next; 
}
```
- [Submission](https://leetcode.com/submissions/detail/1112741766/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| C | 7 ms | 7.3 MB |
```
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
void deleteNode(struct ListNode* node) {
    struct ListNode *prev;
    struct ListNode *next = node->next;
    while (next != NULL) {
        node->val = next->val;
        prev = node;
        node = next;
        next = next->next;
    }
    prev->next = NULL;
}
```
