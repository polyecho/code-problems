# https://www.algoexpert.io/questions/sorted-squared-array


def sortedSquaredArray(array):
    return_value = map(lambda one: one ** 2, array)
    return_value = sorted(return_value)

    return return_value
