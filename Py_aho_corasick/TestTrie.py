class TrieNode:
     
    # Trie node class
    def __init__(self):
        # [None, None, None, None, None, ...] to the index 25
        # where 26 represents the number of letters in the alphabet
        self.children = [None for _ in range(26)]
 
        # isEndOfWord is True if node represent the end of the word
        self.isEndOfWord = False

class Trie:
     
    # Trie data structure class
    def __init__(self):
        self.root = self.getNode()
 
    def getNode(self):
     
        # Returns new trie node (initialized to NULLs)
        return TrieNode()
 
    def _charToIndex(self,ch):
         
        # private helper function
        # Converts key current character into index
        # use only 'a' through 'z' and lower case
         
        return ord(ch)-ord('a')
 
 
    def insert(self,key):
         
        # If not present, inserts key into trie
        # If the key is prefix of trie node,
        # just marks leaf node
        pCrawl = self.root
        length = len(key)
        for level in range(length):
            index = self._charToIndex(key[level])
 
            # if current character is not present
            if not pCrawl.children[index]:
                pCrawl.children[index] = self.getNode()
            pCrawl = pCrawl.children[index]
 
        # mark last node as leaf
        pCrawl.isEndOfWord = True
 
    def search(self, key):
         
        # Search key in the trie
        # Returns true if key presents
        # in trie, else false
        pCrawl = self.root
        length = len(key)
        for level in range(length):
            index = self._charToIndex(key[level])
            if not pCrawl.children[index]:
                level += 1
            else:
                pCrawl = pCrawl.children[index]
 
        return pCrawl.isEndOfWord
 
# driver function
def main():
 
    # Input keys (use only 'a' through 'z' and lower case)
    keys = ["the","a","there","anaswe","any",
            "by","their", "that", "me", "zionyxay"]
    output = ["Not present in trie",
              "Present in trie"]
 
    # Trie object
    # At this moment 't' is just a node
    t = Trie()
    print("this it t : {} ".format(t))
 
    # Construct trie
    for key in keys:
        t.insert(key)
    print("this is t : {}".format(t))
 
    # Search for different keys
    print("{} ---- {}".format("karatekamachinethus",output[t.search("karatekamachinethus")]))
    print("{} ---- {}".format("these",output[t.search("these")]))
    print("{} ---- {}".format("their",output[t.search("their")]))
    print("{} ---- {}".format("thaw",output[t.search("thaw")]))
    print("{} ---- {}".format("that",output[t.search("that")]))
    print("{} ---- {}".format("zionyxay",output[t.search("zionyxay")]))
 
if __name__ == '__main__':
    main()