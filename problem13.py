from functools import reduce

from get_input import get_input_from_file


def parse_timetable(table):
    table = table.split(",")
    table = [int(el) if el != "x" else el for el in table]
    return table


def egcd(a, b):
    if a == 0:
        return b, 0, 1
    else:
        gcd, x, y = egcd(b % a, a)
    return gcd, y - (b//a) * x, x


if __name__ == '__main__':
    data = get_input_from_file("problem13.txt")
    departure_time = int(data[0])
    routes = parse_timetable(data[1])
    # minutes_to_bus = list(map(lambda x: x - (departure_time % x), routes))
    # waiting_time = min(minutes_to_bus)
    # index = minutes_to_bus.index(waiting_time)
    # bus_id = routes[index]
    #
    # print(waiting_time * bus_id)

    # found = False
    # curr_ind = -1
    # while not found:
    #     found = True
    #     curr_ind += 1
    #     t = curr_ind
    #
    #     for route in routes:
    #         if route == "x":
    #             pass
    #         elif not t == departure_time % route:
    #             found = False
    #             break
    #         t += 1
    #
    # print(curr_ind)

    a_ = []
    m_ = []

    for a, m in enumerate(routes):
        if m != "x":
            a_.append(-a)
            m_.append(m)

    M = reduce(lambda x, y: x * y, m_, 1)

    M_ = [M // m_i for m_i in m_]

    r_, s_, e_ = [], [], []
    for m_i, M_i in zip(m_, M_):
        gcd, r_i, s_i = egcd(m_i, M_i)
        r_.append(r_i)
        s_.append(s_i)
        e_.append(s_i * M_i)

    res = 0
    for t_i, e_i in zip(a_, e_):
        res += t_i * e_i
    print(res % M)




