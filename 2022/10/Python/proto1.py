solution = 0
reg = 1
cycle = 0
first_cycle = True

with open("input.txt", "r") as f:
    for line in f.readlines():
        skip_check = False

        if line.startswith("noop"):
            cycle += 1
            print(f"{cycle}: .")

        else:
            cycle += 1
            if first_cycle and cycle % 20 == 0:
                solution += cycle * reg
                first_cycle = False
                print(
                    f"\033[31m{cycle+1}: {reg} --> {(cycle+1) * reg}\033[0m")
                skip_check = True
            elif (cycle - 20) % 40 == 0:
                solution += cycle * reg
                print(
                    f"\033[31m{cycle+1}: {reg} --> {(cycle+1) * reg}\033[0m")
                skip_check = True

            cycle += 1
            if first_cycle and cycle % 20 == 0:
                solution += cycle * reg
                first_cycle = False
                print(
                    f"\033[31m{cycle+1}: {reg} --> {(cycle+1) * reg}\033[0m")
                skip_check = True
            elif (cycle - 20) % 40 == 0:
                solution += cycle * reg
                print(
                    f"\033[31m{cycle+1}: {reg} --> {(cycle+1) * reg}\033[0m")
                skip_check = True

            _, v = line.split()
            print(f"{cycle}: {reg} + {v} = {reg + int(v)}")
            reg += int(v)

        if skip_check:
            continue
        if first_cycle and cycle % 20 == 0:
            solution += cycle * reg
            first_cycle = False
            print(f"\033[34m{cycle}: {reg} --> {cycle * reg}\033[0m")
        elif (cycle - 20) % 40 == 0:
            solution += cycle * reg
            print(f"\033[34m{cycle}: {reg} --> {cycle * reg}\033[0m")


print(solution)
