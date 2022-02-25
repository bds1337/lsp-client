import fileinput

def main():
    while(1):
        for line in fileinput.input():
            print(f"echo {line}")

if __name__ == "__main__":
    print("test lsp is running...")
    main()