def get_the_number() throws ValueError:
    line = input("Please enter a number: ")
    number = int(line)
    return number

def helper() throws ValueError:
    while True:
        # try:
            return get_the_number()
        # except:
            # print("That was not a number!")

try:
    their_number = helper()
except:
    # ...

print(f'Your number doubled is {their_number * 2}')