import csv
import random
import time
import os
import io
import boto3
import logging
from botocore.exceptions import ClientError
from urllib.parse import quote_plus

# Logger setup
t = logging.getLogger()
t.setLevel(logging.INFO)
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

# Constants
SENDERS = [
    "anna@defytime-solutions.com",
    "sarah@defytime-solutions.com",
    "michael@defytime-solutions.com",
    "emily@defytime-solutions.com",
    "jessica@defytime-solutions.com"
]

# S3 Keys
S3_BUCKET = "my-email-app-bucket"
EMAIL_CSV_KEY = "email_valid.csv"
UNSUBSCRIBE_KEY = "unsubscribed.txt"
LAST_RECEIVER_KEY = "last-receiver.txt"
LAST_SENDER_KEY = "last-sender.txt"
TOTAL_SENT_KEY = "total_sent.txt"

# Notification
SQS_NOTIFICATION_EMAIL = "abhiodf9@gmail.com"
UNSUBSCRIBE_BASE_URL = (
    "https://lpv4lifyk9.execute-api.eu-west-1.amazonaws.com/Unsubscribefunction?email="
)

# Campaign config
SUBJECT = "Reverse aging at the cellular level with defytime capsules"
HTML_TEMPLATE_PATH = os.getenv('HTML_CONTENT_PATH', 'email_template.html')
MAX_TOTAL_EMAILS = int(os.getenv('MAX_TOTAL_EMAILS', '500'))
SQS_QUEUE_URL = os.getenv('SQS_QUEUE_URL', '')

# AWS clients
s3 = boto3.client('s3')
sqs = boto3.client('sqs')
ses = boto3.client('ses', region_name='eu-west-1')

# Helpers
def fetch_file_from_s3(bucket, key):
    try:
        obj = s3.get_object(Bucket=bucket, Key=key)
        return obj['Body'].read().decode('utf-8')
    except ClientError as e:
        if e.response['Error']['Code'] == 'NoSuchKey':
            return ''
        logging.error(f"Error fetching {key}: {e}")
        return ''

def write_file_to_s3(bucket, key, content):
    try:
        s3.put_object(Bucket=bucket, Key=key, Body=content.encode('utf-8'))
    except ClientError as e:
        logging.error(f"Error writing {key}: {e}")

def fetch_html(file_path):
    if not os.path.exists(file_path):
        logging.error(f"HTML template not found: {file_path}")
        return "<p>Email content missing.</p>"
    with open(file_path, 'r', encoding='utf-8') as f:
        return f.read()

# Build the batch of recipients
def build_recipient_batch(bucket, key, last_receiver, limit, unsub_set):
    obj = s3.get_object(Bucket=bucket, Key=key)
    reader = csv.DictReader(io.TextIOWrapper(obj['Body'], encoding='utf-8'))
    batch = []
    seen = not last_receiver
    for row in reader:
        email = row.get('Email', '').strip()
        username = row.get('Username', '').strip()
        if not email or not username:
            continue
        if not seen:
            if email == last_receiver:
                seen = True
            continue
        if email.lower() in unsub_set:
            continue
        if len(batch) < limit:
            batch.append((email, username))
        else:
            break
    return batch

# Rotate sender
def next_sender(last):
    if last in SENDERS:
        idx = (SENDERS.index(last) + 1) % len(SENDERS)
        return SENDERS[idx]
    return SENDERS[0]

# Send email
def send_email(source, dest, subject, html_body):
    try:
        resp = ses.send_email(
            Source=source,
            Destination={'ToAddresses': [dest]},
            Message={'Subject': {'Data': subject}, 'Body': {'Html': {'Data': html_body}}}
        )
        logging.info(f"Email sent from {source} to {dest}, MessageId: {resp['MessageId']}")
        return True
    except ClientError as e:
        logging.error(f"Failed to send email to {dest}: {e}")
        return False

# Main logic
def run_daily_emails(event, context):
    logging.info("run_daily_emails start")

    # Load unsubscribe set
    unsub_list = fetch_file_from_s3(S3_BUCKET, UNSUBSCRIBE_KEY).splitlines()
    unsub_set = {u.strip().lower() for u in unsub_list if u.strip()}

    # Compute global unsubscribe count once
    all_obj = s3.get_object(Bucket=S3_BUCKET, Key=EMAIL_CSV_KEY)
    all_reader = csv.DictReader(io.TextIOWrapper(all_obj['Body'], encoding='utf-8'))
    global_unsub_count = 0
    for row in all_reader:
        email = row.get('Email', '').strip().lower()
        if email and email in unsub_set:
            global_unsub_count += 1
    logging.info(f"Global unsub count: {global_unsub_count}")

    # Load state
    last_receiver = fetch_file_from_s3(S3_BUCKET, LAST_RECEIVER_KEY)
    last_sender = fetch_file_from_s3(S3_BUCKET, LAST_SENDER_KEY)
    already_sent = int(fetch_file_from_s3(S3_BUCKET, TOTAL_SENT_KEY) or '0')
    remaining = MAX_TOTAL_EMAILS - already_sent
    if remaining <= 0:
        logging.info("Reached MAX_TOTAL_EMAILS, exiting.")
        return

    # Build batch
    batch = build_recipient_batch(
        S3_BUCKET, EMAIL_CSV_KEY, last_receiver, remaining, unsub_set
    )
    logging.info(f"Batch size: {len(batch)}")
    if not batch:
        logging.info("No recipients to process, exiting.")
        return

    # Prepare sender and template
    sender = next_sender(last_sender)
    html_tpl = fetch_html(HTML_TEMPLATE_PATH)

    # Send loop
    sent_count = 0
    start_time = time.time()
    for email, user in batch:
        if time.time() - start_time > 1 * 60:
            logging.warning("Timeout reached, stopping send loop.")
            break
        unsubscribe_url = UNSUBSCRIBE_BASE_URL + quote_plus(email)
        body = html_tpl.replace('&#64;{{INSTAHANDLE}}', f'&#64;{user}')
        body = body.replace('{{URL}}', unsubscribe_url)
        if send_email(sender, email, SUBJECT, body):
            sent_count += 1
            already_sent += 1
            write_file_to_s3(S3_BUCKET, LAST_RECEIVER_KEY, email)
            write_file_to_s3(S3_BUCKET, LAST_SENDER_KEY, sender)
            sender = next_sender(sender)
            time.sleep(random.uniform(1, 10))

    # Persist sent count
    write_file_to_s3(S3_BUCKET, TOTAL_SENT_KEY, str(already_sent))

    # Completion check
    adjusted = already_sent + global_unsub_count
    logging.info(f"Adjusted count: {adjusted}/{MAX_TOTAL_EMAILS}")
    if adjusted >= MAX_TOTAL_EMAILS:
        logging.info("Campaign complete, sending final notification.")
        send_email(
            sender,
            SQS_NOTIFICATION_EMAIL,
            '✅ Email Batch Complete',
            f'<p>All {MAX_TOTAL_EMAILS} emails accounted (sent + unsub={adjusted}).</p>'
        )
    else:
        logging.info("Not complete, triggering next batch.")
        send_email(
            sender,
            SQS_NOTIFICATION_EMAIL,
            '⏳ Lambda Re-Triggered',
            f'<p>Sent {already_sent} + {global_unsub_count} unsub = {adjusted}/{MAX_TOTAL_EMAILS}.</p>'
        )
        if SQS_QUEUE_URL:
            try:
                sqs.send_message(QueueUrl=SQS_QUEUE_URL, MessageBody='Trigger next batch')
            except ClientError as e:
                logging.error(f"Failed to send SQS trigger: {e}")

    logging.info(f"Finished run: sent_run={sent_count}, total_sent={already_sent}")

# Lambda entry point
def lambda_handler(event, context):
    run_daily_emails(event, context)
    return {'statusCode': 200}
