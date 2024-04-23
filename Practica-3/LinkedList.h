#pragma once

#include "ListNode.h"
#include <iostream>

Class LinkedList {
private:
  ListNode *_phead;

public:
  LinkedList();
  void insert(int n);
  void print(void);

  void deleteAll(void);
  void deleteNodes(ListNode * pn);
};
