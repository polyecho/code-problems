# https://www.algoexpert.io/questions/non-constructible-change


def nonConstructibleChange(coins):
    sorted_coins = sorted(coins)
    current_coin_sum = 0

    for coin in sorted_coins:
        if coin > current_coin_sum + 1:
            return current_coin_sum + 1
        elif coin <= current_coin_sum + 1:
            current_coin_sum += coin

    return current_coin_sum + 1
