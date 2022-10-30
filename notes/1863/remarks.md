Notice that if denote bitwise-EQ (===) operation as BEQ, we have the following formula:
XOR = ADD - (BEQ << 1)
E.g. (11 ^ 6) = (1101b ^ 110b) = 1001b = 9 = 17 - (4 << 1) = (1101b === 110b)
