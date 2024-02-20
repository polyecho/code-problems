# https://www.algoexpert.io/questions/two-number-sum


def twoNumberSum(array, targetSum):
    for i in array:
        for j in array:
            if i == j:
                continue
            elif i + j == targetSum:
                return [i, j]
    return []
