def my_int(x) throws ValueError, IoException, SqlException:
    # all of the code
    # that does `int`
    int(x)

def get_user_number() throws ValueError, IoException, SqlException:
    num = my_int(input("Please enter a number: "))
    return num

def helper1() throws ValueError, IoException, SqlException:
    return get_user_number()

def gives_value():
    return None

def helper2() throws ValueError, IoException, SqlException:
    return helper1()



def helper3():
    while True:
        try:
            return helper2()
        #   catch:
        except ValueError as err:
            print("That's not a number!")
            print(f"The error was: {err}")
        except IoException:
            pass

num = helper3()

def main():
    try:
        run_program()
    except:
        restart_program()

print(f"Double your number is {num * 2}")
