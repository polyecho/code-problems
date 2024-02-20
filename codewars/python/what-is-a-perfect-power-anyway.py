# https://www.codewars.com/kata/54d4c8b08776e4ad92000835


def isPP(n: int):
    for i in range(2, int(n ** 0.5) + 1):
        j = 2

        while i ** j <= n:
            if i ** j == n:
                return [i, j]
            j += 1

    return None
