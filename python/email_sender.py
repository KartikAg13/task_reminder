import datetime
from email.mime.text import MIMEText
import smtplib
import os
import json

def send_email(recipient, message):

    sender_email = os.environ.get('EMAIL_SENDER')
    password = os.environ.get('EMAIL_PASSWORD')

    msg = MIMEText(message)
    msg["Subject"] = "Task Reminder"
    msg["From"] = sender_email
    msg["To"] = recipient

    with smtplib.SMTP_SSL("smtp.gmail.com", 465) as server:
        server.login(sender_email, password)
        server.sendmail(sender_email, recipient, msg.as_string())
        print(f"Email sent successfully on {datetime.datetime.now()}")


def send_reminders(task_names, task_descriptions, frequency):

    with open('/home/kartik/Desktop/task_reminder/src/data.json', 'r') as file:
        json_data = json.load(file)
    user_email = json_data[0]["email"]

    for i, task_name in enumerate(task_names):
        description = task_descriptions[i] if task_descriptions[i] else "No Description"
        message = f"Reminder: '{task_name}' is due in {frequency} days. {description}"
        send_email(user_email, message)
        print(f"Email sent (reminder for '{task_name}')")
