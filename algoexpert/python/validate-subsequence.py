# https://www.algoexpert.io/questions/validate-subsequence


def isValidSubsequence(array, sequence):
    array_selected = []
    sequence_last_found_i = 0

    for array_e in array:
        for (i, item) in enumerate(sequence, start=sequence_last_found_i):
            if array_e == item:
                array_selected.append(item)
                sequence_last_found_i = i
                break

        if array_selected == sequence:
            return True

    return False
