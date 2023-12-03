import sys
import json
from ouroboros import init  # Assuming 'init' is in a module named 'ouroboros'

def main():
    input_data, output_data = init(list[int], int)

    # Assuming input_data is a list of integers and we sum them up
    result = sum(input_data)

    # Writing the result to standard output in JSON format
    sys.stdout.write(json.dumps(result))

if __name__ == "__main__":
    main()
