# https://www.algoexpert.io/questions/tournament-winner


def tournamentWinner(competitions, results):
    home_list = []
    away_list = []

    for r_index, r_item in enumerate(results):
        if r_item == 0:
            home_list.append(competitions[r_index][r_item + 1])
            away_list.append(competitions[r_index][r_item])
        elif r_item == 1:
            home_list.append(competitions[r_index][r_item - 1])
            away_list.append(competitions[r_index][r_item])

    final_winner = None
    winner_arr = []

    for one in home_list:
        if len(winner_arr) == 0:
            winner_arr.append({"name": one, "count": 0})
        else:
            is_found = False

            for two in winner_arr:
                if two["name"] == one:
                    is_found = True
                    two["count"] += 1

            if is_found is False:
                winner_arr.append({"name": one, "count": 0})

    for one in winner_arr:
        if final_winner is None:
            final_winner = one
        else:
            if one["count"] > final_winner["count"]:
                final_winner = one

    return final_winner["name"]
