import csv
import random
import time
import os
import io
from datetime import datetime
import boto3
from botocore.exceptions import ClientError
import logging

from urllib.parse import quote_plus

# Logger setup
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

# Constants
SENDERS = [
    "anna@defytime-solutions.com",
    "sarah@defytime-solutions.com",
    "michael@defytime-solutions.com",
    "emily@defytime-solutions.com",
    "jessica@defytime-solutions.com"
]

S3_BUCKET = "my-email-app-bucket"
EMAIL_CSV_KEY = "email_valid.csv"
LAST_RECEIVER_KEY = "last-receiver.txt"
LAST_SENDER_KEY = "last-sender.txt"
UNSENT_KEY = "unsent.txt"
TOTAL_SENT_KEY = "total_sent.txt"
TOTAL_UNSUB_KEY = "total_unsub.txt"
LAST_RECEIVER_NOTIFICATION_EMAIL = "narottam2363@gmail.com"
SQS_NOTIFICATION_EMAIL = "abhiodf9@gmail.com"
UNSUBSCRIBE_BASE_URL = (
    "https://lpv4lifyk9.execute-api.eu-west-1.amazonaws.com/Unsubscribefunction?email="
)

# AWS clients
s3 = boto3.client('s3')
sqs = boto3.client('sqs')
ses = boto3.client('ses', region_name='eu-west-1')

# Environment variables
MAX_TOTAL_EMAILS = int(os.getenv('MAX_TOTAL_EMAILS', '500'))
SQS_QUEUE_URL = os.getenv('SQS_QUEUE_URL')

# S3 file utils
def fetch_file_from_s3(bucket, key):
    try:
        response = s3.get_object(Bucket=bucket, Key=key)
        return response['Body'].read().decode('utf-8').strip()
    except ClientError as e:
        if e.response['Error']['Code'] == 'NoSuchKey':
            return ""
        logging.error(f"Failed to fetch {key} from S3: {e}")
        return ""

def write_file_to_s3(bucket, key, content):
    try:
        s3.put_object(Bucket=bucket, Key=key, Body=content.encode('utf-8'))
    except ClientError as e:
        logging.error(f"Failed to write {key} to S3: {e}")

def append_to_s3_file(bucket, key, new_lines):
    existing = fetch_file_from_s3(bucket, key)
    combined = existing + "\n" + "\n".join(new_lines) if existing else "\n".join(new_lines)
    write_file_to_s3(bucket, key, combined)

# CSV parsing
def fetch_email_username_pairs(bucket, key):
    content = fetch_file_from_s3(bucket, key)
    pairs = []
    reader = csv.DictReader(io.StringIO(content))
    for row in reader:
        email = row.get("Email", "").strip()
        username = row.get("Username", "").strip()
        if email and username:
            pairs.append((email, username))
    return pairs

# HTML template
def fetch_html(file_path):
    if not os.path.exists(file_path):
        logging.error(f"HTML file not found at: {file_path}")
        return "<p>Email content missing.</p>"
    with open(file_path, 'r') as file:
        return file.read()

# Personalization
def get_unsubscribe_url(email):
    return UNSUBSCRIBE_BASE_URL + quote_plus(email)

# Unsubscribe check
def is_unsubscribed(email):
    unsubscribed_list = fetch_file_from_s3(S3_BUCKET, 'unsubscribed.txt')
    unsubscribed_emails = set(filter(None, map(str.strip, unsubscribed_list.split('\n'))))
    return email.lower() in unsubscribed_emails

# Email sending
def send_email(sender, recipient, subject, body_html):
    try:
        response = ses.send_email(
            Source=sender,
            Destination={'ToAddresses': [recipient]},
            Message={
                'Subject': {'Data': subject},
                'Body': {'Html': {'Data': body_html}}
            },
            ConfigurationSetName='DefytimeConfigSet'
        )
        logging.info(f"Email sent from {sender} to {recipient}. MessageId: {response['MessageId']}")
        return True
    except ClientError as e:
        logging.error(f"Failed to send from {sender} to {recipient}: {e.response['Error']['Message']}")
        return False

# Sender rotation
def get_next_sender(last_email):
    if last_email in SENDERS:
        index = (SENDERS.index(last_email) + 1) % len(SENDERS)
    else:
        index = 0
    return SENDERS[index]

# Main function
def run_daily_emails(event, context):
    # Load state
    email_username_pairs = fetch_email_username_pairs(S3_BUCKET, EMAIL_CSV_KEY)
    if not email_username_pairs:
        logging.warning("No valid email/username pairs found.")
        return

    last_receiver = fetch_file_from_s3(S3_BUCKET, LAST_RECEIVER_KEY)
    already_sent = int(fetch_file_from_s3(S3_BUCKET, TOTAL_SENT_KEY) or "0")
    total_unsub_before = int(fetch_file_from_s3(S3_BUCKET, TOTAL_UNSUB_KEY) or "0")
    remaining_to_send = MAX_TOTAL_EMAILS - already_sent

    if remaining_to_send <= 0:
        logging.info("All emails already sent. Exiting.")
        return

    # Determine batch start index
    emails = [e for e, _ in email_username_pairs]
    if last_receiver and last_receiver in emails:
        start_idx = emails.index(last_receiver) + 1
    else:
        start_idx = 0

    # Build batch (skip unsubscribes) and count unsubscribes
    batch, val_unsub = [], 0
    idx = start_idx
    while idx < len(email_username_pairs) and len(batch) < remaining_to_send:
        email, username = email_username_pairs[idx]
        idx += 1
        if is_unsubscribed(email):
            val_unsub += 1
            write_file_to_s3(S3_BUCKET, LAST_RECEIVER_KEY, email)
            continue
        batch.append((email, username))

    if not batch and val_unsub == 0:
        logging.info("No receivers left to process.")
        return

    # Update total unsub count persistently
    total_unsub = total_unsub_before + val_unsub
    write_file_to_s3(S3_BUCKET, TOTAL_UNSUB_KEY, str(total_unsub))

    # Prepare sender rotation
    last_sender = fetch_file_from_s3(S3_BUCKET, LAST_SENDER_KEY)
    current_sender = get_next_sender(last_sender)

    base_html_content = fetch_html(os.getenv('HTML_CONTENT_PATH', 'email_template.html'))

    # Send loop
    start_time = time.time()
    max_duration = 1 * 60
    sent_count = 0
    new_unsent = []

    for email, username in batch:
        if time.time() - start_time > max_duration:
            logging.warning("Time limit reached. Stopping early.")
            break

        personalized_html = base_html_content.replace('&#64;{{INSTAHANDLE}}', f'&#64;{username}').replace('{{URL}}', get_unsubscribe_url(email))

        if send_email(current_sender, email, subject="Reverse aging at the cellular level with defytime capsules", body_html=personalized_html):
            sent_count += 1
            already_sent += 1
            write_file_to_s3(S3_BUCKET, LAST_RECEIVER_KEY, email)
            write_file_to_s3(S3_BUCKET, LAST_SENDER_KEY, current_sender)
            current_sender = get_next_sender(current_sender)
            time.sleep(random.uniform(1, 10))
        else:
            new_unsent.append(email)

    write_file_to_s3(S3_BUCKET, TOTAL_SENT_KEY, str(already_sent))
    if new_unsent:
        append_to_s3_file(S3_BUCKET, UNSENT_KEY, new_unsent)

    # Determine global adjusted count
    adjusted_sent = already_sent + total_unsub
    logging.info(f"Final counts — sent: {already_sent}, unsubscribed total: {total_unsub}, overall: {adjusted_sent}/{MAX_TOTAL_EMAILS}")

    # Notification logic
    if adjusted_sent >= MAX_TOTAL_EMAILS:
        send_email(
            current_sender,
            SQS_NOTIFICATION_EMAIL,
            "✅ Email Batch Complete",
            f"<p>All {MAX_TOTAL_EMAILS} emails accounted for (sent + unsubscribed = {adjusted_sent}).</p>"
        )
    else:
        try:
            sqs.send_message(QueueUrl=SQS_QUEUE_URL, MessageBody="Trigger next email batch")
            send_email(
                current_sender,
                SQS_NOTIFICATION_EMAIL,
                "⏳ Lambda Re-Triggered",
                f"<p>Sent {already_sent} emails + {total_unsub} unsubscribes = {adjusted_sent}/{MAX_TOTAL_EMAILS}. Triggered next batch.</p>"
            )
        except ClientError as e:
            logging.error(f"Failed to trigger SQS: {e}")

    logging.info(f"Finished. Run sent: {sent_count}, Overall sent: {already_sent}, total unsub: {total_unsub}")

# Lambda entry
def lambda_handler(event, context):
    run_daily_emails(event, context)
