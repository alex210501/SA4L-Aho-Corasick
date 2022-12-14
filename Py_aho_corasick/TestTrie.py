class TrieNode:
     
    # Trie node class
    def __init__(self):
        # [None, None, None, None, None, ...] to the index 25
        # where 26 represents the number of letters in the alphabet
        self.children = [None for _ in range(26)]

        self.failure_link = None

        self.dictionnary_link = None

        self.parent = None
 
        # isEndOfWord is True if node represent the end of the word
        self.isEndOfWord = False

class Trie:
     
    # Trie data structure class
    def __init__(self):
        self.root = self.getNode()
        self.root.failure_link = self.root
 
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

            # Referencing the parent of the new node
            pCrawl.children[index].parent = pCrawl
            pCrawl.children[index].parent.isEndOfWord = False
            # Going to the new node
            pCrawl = pCrawl.children[index]

 
        # mark last node as leaf
        pCrawl.isEndOfWord = True
 
    def search(self, key):
         
        # Search key in the trie
        # Returns true if key presents
        # in trie, else false
        pCrawl = self.root
        length = len(key)
        letters_present = []
        for level in range(length):
            index = self._charToIndex(key[level])
            # If the letter isn't present in the trie (not contained in the root's children)
            if not pCrawl.children[index] and pCrawl is self.root:
                pass

            # If the letter isn't present in the actual branch
            elif not pCrawl.children[index]:
                pCrawl = pCrawl.failure_link
                # If the letter is present in the failure link's children
                if pCrawl.children[index]:
                    letters_present.append(key[level])
                    """
                    if pCrawl.children[index].isEndOfWord:
                        print("this is letters presents first : {}".format(letters_present)) 
                        return pCrawl.children[index].isEndOfWord
                    """
            else:
                pCrawl = pCrawl.children[index]
                letters_present.append(key[level])
                print("Is end of word check for this letter {} : {}".format(key[level], pCrawl.isEndOfWord == True))
                """
                if pCrawl.isEndOfWord:
                    print("this is letters presents second : {}".format(letters_present)) 
                    return pCrawl.isEndOfWord
                """

                
        print("this is letters presents third : {}".format(letters_present)) 
        return pCrawl.isEndOfWord
    
    def failure_links(self):
        # Function that will go through the trie 
        # and that will build the failure links and 
        # the dictionnary links

        visited = []
        queue = []

        pCrawl = self.root
        visited.append(pCrawl)
        queue.append(pCrawl)

        while queue:

            node = queue.pop(0)
            length = len(node.children)

            for index in range(length):

                if node.children[index] is not None:

                    if node.children[index] not in visited:

                        visited.append(node.children[index])
                        queue.append(node.children[index])
                        # If my actual node is directly related to the root
                        # his failure link will be root
                        if node is self.root:         
                            node.children[index].failure_link = self.root

                        # If my actual node isn't directly related to the root
                        # go check his parent's failure link
                        # if this node (the node of the failure link)
                        # contains a chil that is the same as my actual node
                        # my actual node's failure link will be this node
                        else:
                            try:
                                # pCrawl is the failure link
                                pCrawl = node.failure_link
                                if pCrawl.children[index]:
                                    node.children[index].failure_link = pCrawl.children[index]
                                else:
                                    if self.root.children[index]:
                                        node.children[index].failure_link = self.root.children[index]
                                    else:
                                        node.children[index].failure_link = self.root
                            except:
                                return False
        print("The failure links have been built")
        return True
 
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

    # Construct the failure links in the trie
    t.failure_links()
 
    # Search for different keys

    print("{} ---- {}".format("anrlphabtheirmot",output[t.search("anrlphabtheirmot")]))
    """
    print("{} ---- {}".format("these",output[t.search("these")]))
    print("{} ---- {}".format("their",output[t.search("their")]))
    print("{} ---- {}".format("thaw",output[t.search("thaw")]))
    print("{} ---- {}".format("that",output[t.search("that")]))
    print("{} ---- {}".format("zionyxay",output[t.search("zionyxay")]))
    """

 
if __name__ == '__main__':
    main()