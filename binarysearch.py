import time

def binary_search(arr, target):
    low = 0
    high = len(arr) - 1
    tries = 0
    
    while low <= high:
        tries += 1
        mid = (low + high) // 2
        if arr[mid] == target:
            return mid, tries
        elif arr[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    
    return -1, tries

def main():
    # Get the maximum value for the series of numbers
    max_value = int(input("Enter the maximum value for the series (1-X): "))

    # Generate the list of numbers from 1 to max_value
    numbers = list(range(1, max_value + 1))

    # Ask for the number to search for
    target = int(input(f"Enter a number between 1 and {max_value} to search for: "))

    # Start time
    start_time = time.time()

    # Perform binary search
    index, tries = binary_search(numbers, target)

    # End time
    end_time = time.time()

    # Calculate the duration of the search
    duration = end_time - start_time

    # Print the result
    if index != -1:
        print(f"Number {target} found at index {index} after {tries} tries.")
    else:
        print(f"Number {target} not found after {tries} tries.")

    print(f"Time taken to search: {duration:.6f} seconds.")

if __name__ == "__main__":
    main()
