data = []
with open('d8\data.txt', 'r') as f:
    for l in f:
        data.append(l.strip().split(' | '))

ans = 0
for x in data:
    digits = ['' for _ in range(10)]
    a, b = x

    while not all(digits):
        for signal in a.split():
            slen = len(signal)
            if slen == 2:
                digits[1] = signal
            elif slen == 3:
                digits[7] = signal
            elif slen == 4:
                digits[4] = signal
            elif slen == 7:
                digits[8] = signal
            elif slen == 5:
                if digits[3] and digits[5]:
                    if not all(c in digits[3] for c in signal) and not all(c in digits[5] for c in signal):
                        digits[2] = signal
                if digits[1] and all(c in signal for c in digits[1]):
                    digits[3] = signal
                if digits[6] and all(c in digits[6] for c in signal):
                    digits[5] = signal
            elif slen == 6:
                if digits[1] and digits[3] and digits[4] and digits[8] and digits[9]:
                    temp = digits[9]
                    for z in digits[9]:
                        if z in digits[4]:
                            temp = temp.replace(z, '')

                    zero = digits[1] + temp
                    temp = digits[8]
                    for z in digits[8]:
                        if z in digits[3]:
                            temp = temp.replace(z, '')
                    zero += temp

                    digits[0] = zero
                if digits[4] and all(c in signal for c in digits[4]):
                    digits[9] = signal
                if digits[0] and digits[9]:
                    if not all(c in digits[0] for c in signal) and not all(c in digits[9] for c in signal):
                        digits[6] = signal

    num = ''
    for output in b.split():
        for i, dig in enumerate(digits):
            if len(output) == len(dig) and all(c in dig for c in output):
                num += str(i)

    ans += int(num)

print(ans)
