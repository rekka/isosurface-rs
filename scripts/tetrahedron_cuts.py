def recur(idx):
    n = len(idx)
    indent = ' ' * (4 * n)
    if len(idx) < 4:
        print(indent + f'if u{n} >= zero {{')
        if len(idx) < 3:
            print(indent + f'    let u{n} = u{n}.nudge();')
        idx.append(0)
        recur(idx)
        idx.pop()
        print(indent + '} else {')
        idx.append(1)
        recur(idx)
        idx.pop()
        print(indent + '}')
        return

    n = sum(idx)

    if n == 0 or n == 4:
        return

    zero = []
    one = []
    for i in range(0, 4):
        if idx[i]:
            one.append(i)
        else:
            zero.append(i)

    for i in one:
        for j in zero:
            print(indent + f'emit_vertex(v{i}.interpolate(&v{j}, u{i}, u{j}));')

    if n == 2:
        print(indent + 'emit_face([0, 1, 2]);')
        print(indent + 'emit_face([2, 1, 3]);')
    elif n == 1 or n == 3:
        print(indent + 'emit_face([0, 1, 2]);')

print('// START GENERATED: generated by `scripts/tetrahedron_cuts.py`')
recur([])
print('// END GENERATED')
