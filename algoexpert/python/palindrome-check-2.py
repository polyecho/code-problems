# https://www.algoexpert.io/questions/palindrome-check


def isPalindrome(string):
    str_length = (len(string) // 2) + 1

    for i in range(str_length):
        if string[i] != string[-i - 1]:
            return False

    return True
