# run with: `python3 optional.py`

def create(b):
    if b:
        return "Hello there"
    else:
        return None

def main():
    # Method 1
    create_true = create(True)
    if create_true:
        print("create(True) returned " + create_true)

    # Method 2
    create_false = create(False)
    print(
        "create(False) returned "
        + create_false if create_false else "<empty>"
    )

if __name__ == "__main__":
    main()
