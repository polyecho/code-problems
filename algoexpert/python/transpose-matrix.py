# https://www.algoexpert.io/questions/transpose-matrix


def transposeMatrix(matrix):
    merged_list = []

    col_length = len(matrix[0])
    for i_col in range(col_length):
        for i_row in range(len(matrix)):
            merged_list.append(matrix[i_row][i_col])

    new_matrix = []
    new_matrix_row_length = len(matrix)

    new_matrix_row_number = 0

    for _, item in enumerate(merged_list):
        if new_matrix != []:
            new_matrix_row_number = len(new_matrix[len(new_matrix) - 1])

        if new_matrix_row_number == 0 or new_matrix_row_number >= new_matrix_row_length:
            new_matrix.append([])

        new_matrix[len(new_matrix) - 1].append(item)

    return new_matrix
