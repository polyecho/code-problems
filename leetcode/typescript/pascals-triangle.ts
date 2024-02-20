// https://leetcode.com/problems/pascals-triangle/

function generate(numRows: number): number[][] {
    const returnValue: number[][] = [];

    for (let i = 0; i < numRows; i++) {
        const numberArr: number[] = [];

        for (let j = 0; j < i + 1; j++) {
            if (i < 2) {
                // * Iterate rows.
                // Push 1 when none of the two conditions should work
                numberArr.push(1);
            } else {
                if (j >= 0 + 1 && j <= i - 1) {
                    // * Iterate Columns.
                    const pushValue =
                        returnValue[i - 1][j - 1] + returnValue[i - 1][j];

                    // Insert the valid value for specific conditions.
                    numberArr.push(pushValue);
                } else {
                    numberArr.push(1);
                }
            }
        }

        if (numberArr.length > 0 && numberArr.length === i + 1) {
            returnValue.push(numberArr);
        }
    }

    return returnValue;
}
