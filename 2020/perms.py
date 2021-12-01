def to_list(s):
    return list(map(int, s.split()))


def check_permutations(li, perms):
    for p in perms:
        if li == p[0]:
            return str(p[1])
    return "Too chaotic"


permutations = [([1, 2, 3, 4, 5, 6, 7, 8], 0), ([2, 1, 3, 4, 5, 6, 7, 8], 1)]

# curr_ind = 2
# while curr_ind < 8:
#     new_perms = []
#     for perm in permutations:
#         curr = perm[0].copy()
#         curr[curr_ind-1], curr[curr_ind] = curr[curr_ind], curr[curr_ind - 1]
#         new_perms.append((curr, perm[1] + 1))
#         curr = perm[0].copy()
#         curr[curr_ind - 2], curr[curr_ind] = curr[curr_ind], curr[curr_ind - 2]
#         new_perms.append((curr, perm[1] + 2))
#     permutations.extend(new_perms)
#
#     curr_ind += 1


def num_bribes(num, subl):
    res = 0
    for s in subl:
        if num > s:
            res += 1
    return res


ground = [i for i in range(8)]
inp = to_list("5 1 2 3 7 8 6 4")
for i in range(len(inp)):
    inp[i] -= 1

swaps = 0
for i, x in enumerate(inp):
    bribes = num_bribes(x, inp[i+1:])
    if bribes > 2:
        print("Too chaotic")
        break
    swaps += bribes
print(swaps)



