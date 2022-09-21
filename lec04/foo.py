def get_user_number():
    num = int(input("Please enter a number: "))
    return num

def helper1():
    return get_user_number()

def gives_value():
    return None

def helper2():
    x = gives_value()

    try:
        y = x + 5
    except TypeError:
        pass

    return helper1()

def helper3():
    while True:
        try:
            return helper2()
        #   catch:
        except ValueError as err:
            print("That's not a number!")
            print(f"The error was: {err}")

num = helper3()

def main():
    try:
        run_program()
    except:
        restart_program()

print(f"Double your number is {num * 2}")
