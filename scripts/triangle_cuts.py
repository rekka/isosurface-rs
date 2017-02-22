def recur(idx):
    n = len(idx)
    if len(idx) < 3:
        print 'if u' + str(n) + ' >= 0. {'
        if len(idx) < 2:
            print '    let u' + str(n) + ' = u' + str(n) + ' + tiny;'
        idx.append(0)
        recur(idx)
        idx.pop()
        print('} else {')
        idx.append(1)
        recur(idx)
        idx.pop()
        print('}')
        return

    n = sum(idx)

    if n == 0 or n == 3:
        return

    zero = []
    one = []
    for i in range(0, 3):
        if idx[i]:
            one.append(i)
        else:
            zero.append(i)

    pairs = []
    for i in one:
        for j in zero:
            p = sorted([i, j])
            if p[1] - p[0] == 2:
                e = 2
            else:
                e = p[0]
            pairs.append([e, 'interpolate(u{i}, u{j}, v{i}, v{j})'.format(i = p[0], j = p[1])])

    pairs.sort()
    print('emit_line(({}, {}), ({}, {}));'.format(pairs[0][0], pairs[1][0], pairs[0][1], pairs[1][1]))

print('// START GENERATED: generated by `scripts/triangle_cuts.py`')
print('let tiny = 1e-15;')
for i in range(0, 3):
    print('let u{0} = u[{0}];'.format(i))
for i in range(0, 3):
    print('let v{0} = v[{0}];'.format(i))
recur([])
print('// END GENERATED')
