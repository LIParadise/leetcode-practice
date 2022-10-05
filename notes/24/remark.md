# 24. Swap Nodes in Pairs

Dealing with ownership is frightening at first, but in this case it's much simpler: think of the list as a deck of cards used for solitaire. Since rust is move-semantics or transfer ownership by default i.e. if type is not `Copy`, think of the `ListNode` as deck of cards in a solitaire game, in which each deck of cards are required to be under some *placeholder*, in this case just new variable names on the stack. To preserve validity of a solitaire game, one moves decks in between only placeholders; to preserve the list without accidentally drop data, remember to move them back to the main list!

[my leetcode post](https://leetcode.com/problems/swap-nodes-in-pairs/discuss/2663637/Solitaire-is-a-good-game-in-understanding-Rust) which is basically the same paragraph.
