
class Solution:
    
    def max_num():
        return 9
    
    def longestCommonPrefix(self, strs):
        if len(strs) == 0:
            return ""
        if len(strs) == 1:
            return strs[0]
        loop = True
        pos = 0
        while loop:
            curr = None
            for i in strs:
                if len(i) == pos:
                    loop = False
                    pos -= 1
                    break
                if curr is None:
                    curr = i[pos]
                elif curr != i[pos]:
                    loop = False
                    pos -= 1
                    break
            pos += 1
        return strs[0][:pos]


class Tests:

    def test_max_num():
        print("success")