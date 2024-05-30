# 138. Copy List with Random Pointer
- [Submission](https://leetcode.com/submissions/detail/1269446073/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| C | 10 ms | 8.1 MB |
```
struct Node* copyRandomList(struct Node* head) {
    int size = sizeof(struct Node);
    if (!head) return NULL;

    struct Node *new_head = (struct Node*) malloc(size);
    new_head->val = head->val; new_head->random = head->random; new_head->next = NULL;
    head->random = new_head;
    
    struct Node *new, *iter = new_head;
    for(struct Node *node = head->next; node; node = node->next) {
        new = (struct Node*) malloc(size);
        new->val = node->val; new->random = node->random;
        node->random = new;
        iter->next = new; iter = new;
    }
    iter->next = NULL;
    
    for(iter = new_head; iter; iter = iter->next)
        if (iter->random) iter->random = iter->random->random;
    return new_head;
}
```
