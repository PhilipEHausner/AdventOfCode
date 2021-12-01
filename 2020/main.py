def index_of_next_different_char(s: str, start_index: int, char: str) -> int:
    result = -1
    for i, c in enumerate(s[start_index:]):
        if c != char:
            result = i
            break
    if result != -1:
        return result + start_index
    else:
        return result


def longest_two_char_substring(s: str) -> int:
    if len(s) < 3:
        return len(s)

    curr_max_len = 2

    start = 0
    end = index_of_next_different_char(s, start, s[start])

    if end == -1:
        return len(s)

    curr_chars = (s[start], s[end])
    last_seen_char = s[end]
    last_seen_char_pos = end

    while end != -1:
        last_seen_char_pos = end
        last_seen_char = s[end]

        end = index_of_next_different_char(s, end, last_seen_char)

        if end == -1:
            break

        if s[end] in curr_chars:
            continue

        else:
            curr_max_len = max(curr_max_len, end - start)
            start = last_seen_char_pos
            curr_chars = (s[start], s[end])

    curr_max_len = max(curr_max_len, len(s) - start)

    return curr_max_len


if __name__ == "__main__":
    print(longest_two_char_substring("aaabbcabba"))
