import heapq


if __name__ == '__main__':
    num_queries = input().strip()
    heap = []
    heapq.heapify(heap)

    for _ in range(int(num_queries)):
        query = input().split(' ')
        query_type = int(query[0])
        query_val = query[1] if len(query) == 2 else None

        # insert
        if query_type == 1:
            heapq.heappush(heap, int(query_val))
        elif query_type == 2:
            heap.remove(int(query_val))
        elif query_type == 3:
            print(heap[0])
