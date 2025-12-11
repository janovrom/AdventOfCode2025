from day10.solution import Int, Optimize, sat

def solve_buttons(target, buttons):
    n = len(target)
    m = len(buttons)
    # Convert button definitions into coefficient matrix B[i][j]
    B = [[0]*m for _ in range(n)]
    for j, btn in enumerate(buttons):
        for i in btn:
            B[i][j] += 1
    # Z3 integer variables: x_j = number of presses of button j
    x = [Int(f"x_{j}") for j in range(m)]
    opt = Optimize()
    # Variables must be nonnegative integers
    for j in range(m):
        opt.add(x[j] >= 0)
    # Constraints: B * x = target
    for i in range(n):
        opt.add(sum(B[i][j] * x[j] for j in range(m)) == target[i])
    # Objective: minimize total presses
    opt.minimize(sum(x))
    if opt.check() != sat:
        print("No solution")
        return None
    mod = opt.model()
    solution = [mod[x[j]].as_long() for j in range(m)]
    return solution

def part2():
    res = 0
    with open("./input/day10.txt") as f:
        for line in f:
            segs = line.split()
            buttons = [eval(f'[{b.strip("()")}]') for b in segs[1:-1]]
            target = eval(f'[{segs[-1].strip("{}")}]')
            res += sum(solve_buttons(target, buttons))
    print(res)

if __name__ == "__main__":
    part2()
