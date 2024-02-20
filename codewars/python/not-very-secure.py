# https://www.codewars.com/kata/526dbd6c8c0eb53254000110


def alphanumeric(password: str):
    if len(password) < 1:
        return False

    for item in password:
        if not (item.isalpha() or item.isdigit()):
            return False

    return True
