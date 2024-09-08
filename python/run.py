import json
import sys
import os
import email_sender

def main():
    os.system("clear")

    with open('/home/kartik/Desktop/task_reminder/src/data.json', 'r') as file:
        json_data = json.load(file)

    task_names = []
    task_descriptions = []
    for user_data in json_data:
        task_names.append(user_data['task']['name'])
        task_descriptions.append(user_data['task']['description'])

    while True:
        enable_email = int(input("Do you want email reminders?\n0. Exit\n1. Yes\n2. No\n"))
        if enable_email == 0:
            print("Thank you")
            sys.exit(0)
        elif enable_email == 1:
            frequency = input("Please enter the frequency at which you would like to get the reminders in days: ")
            email_sender.send_reminders(task_names, task_descriptions, frequency)  # Call send_reminders function
            break
        elif enable_email == 2:
            print("Ok")
            sys.exit(0)
        else:
            continue

if __name__ == "__main__":
    main()