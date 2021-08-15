#!/bin/python3
# https://www.hackerrank.com/challenges/sock-merchant/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=warmup

def main():
    socks = input().split(' ')

    # { sock_type: num_socks }
    sock_map = {}

    # could just do this with list/dict composition, but i didnt feel like it
    for sock in socks:
        if sock in sock_map:
            sock_map[sock] = sock_map[sock] + 1
        else:
            sock_map[sock] = 1

    print(sock_map)

    num_pairs = 0
    for sock_type in sock_map:
        num_pairs = num_pairs + int(sock_map[sock_type] / 2)

    print(num_pairs)


if __name__ == '__main__':
    main()
