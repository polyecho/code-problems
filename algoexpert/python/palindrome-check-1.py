# https://www.algoexpert.io/questions/palindrome-check


def isPalindrome(string: str):
    reversed_string = ""

    for item in reversed(string):
        reversed_string += item

    if string == reversed_string:
        return True
    else:
        return False
