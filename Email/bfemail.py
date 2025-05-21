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

S3_BUCKET = "my-email-app-bucket"
EMAIL_CSV_KEY = "email_valid.csv"
LAST_RECEIVER_KEY = "last-receiver.txt"
LAST_SENDER_KEY = "last-sender.txt"
UNSENT_KEY = "unsent.txt"
TOTAL_SENT_KEY = "total_sent.txt"
UNSUBSCRIBE_KEY = "unsubscribed.txt"
SQS_NOTIFICATION_EMAIL = "abhiodf9@gmail.com"
UNSUBSCRIBE_BASE_URL = (
    "https://lpv4lifyk9.execute-api.eu-west-1.amazonaws.com/Unsubscribefunction?email="
)
# Email config
# Subject for all outgoing campaign emails
SUBJECT = "Reverse aging at the cellular level with defytime capsules"
# Path to HTML template (loaded from local container at runtime)
HTML_TEMPLATE_PATH = os.getenv('HTML_CONTENT_PATH', 'email_template.html')

# AWS clients
s3 = boto3.client('s3')
sqs = boto3.client('sqs')
ses = boto3.client('ses', region_name='eu-west-1')

# Environment variables
MAX_TOTAL_EMAILS = int(os.getenv('MAX_TOTAL_EMAILS', '500'))
SQS_QUEUE_URL = os.getenv('SQS_QUEUE_URL')

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
        s3.put_object(Bucket=bucket, Key=key, Body=content)
    except ClientError as e:
        logging.error(f"Error writing {key}: {e}")

def append_to_s3(bucket, key, lines):
    existing = fetch_file_from_s3(bucket, key)
    new_content = existing + ''.join(line + '\n' for line in lines)
    write_file_to_s3(bucket, key, new_content)

def fetch_html(file_path):
    """Load HTML template from local file system or return placeholder."""
    if not os.path.exists(file_path):
        logging.error(f"HTML template not found at {file_path}")
        return "<p>Email content missing.</p>"
    with open(file_path, 'r') as f:
        return f.read()

# Stream and batch
def stream_and_build_batch(bucket, key, last_receiver, remaining, unsub_set):
    obj = s3.get_object(Bucket=bucket, Key=key)
    reader = csv.DictReader(io.TextIOWrapper(obj['Body'], encoding='utf-8'))
    batch = []
    global_unsub = 0
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
            global_unsub += 1
            continue
        if len(batch) < remaining:
            batch.append((email, username))
    return batch, global_unsub

# Email send
def send_email(src, dst, subject, html):
    try:
        res = ses.send_email(
            Source=src,
            Destination={'ToAddresses': [dst]},
            Message={'Subject': {'Data': subject}, 'Body': {'Html': {'Data': html}}}
        )
        logging.info(f"Sent from {src} to {dst} id={res['MessageId']}")
        return True
    except ClientError as e:
        logging.error(f"Send error: {e}")
        return False

# Rotate sender
def next_sender(last):
    if last in SENDERS:
        idx = (SENDERS.index(last) + 1) % len(SENDERS)
        return SENDERS[idx]
    return SENDERS[0]

# Main logic
def run_daily_emails(event, context):
    logging.info("run_daily_emails start")

    # build unsubscribe set
    unsub_list = fetch_file_from_s3(S3_BUCKET, UNSUBSCRIBE_KEY).splitlines()
    unsub_set = {u.strip().lower() for u in unsub_list if u.strip()}

    # load state
    last_receiver = fetch_file_from_s3(S3_BUCKET, LAST_RECEIVER_KEY)
    already_sent = int(fetch_file_from_s3(S3_BUCKET, TOTAL_SENT_KEY) or '0')
    remaining = MAX_TOTAL_EMAILS - already_sent
    if remaining <= 0:
        logging.info("All emails sent. Exiting.")
        return

    # build batch + count unsubscribes
    batch, global_unsub = stream_and_build_batch(
        S3_BUCKET, EMAIL_CSV_KEY, last_receiver, remaining, unsub_set
    )
    logging.info(f"Batch size={len(batch)}, unsub_count={global_unsub}")
    if not batch and global_unsub == 0:
        logging.info("No recipients or unsubscribes. Exiting.")
        return

    # rotate sender and load template
    last_sender = fetch_file_from_s3(S3_BUCKET, LAST_SENDER_KEY)
    sender = next_sender(last_sender)
    html_tpl = fetch_html(HTML_TEMPLATE_PATH)  # HTML loaded from this file path

    # send loop
    sent_count = 0
    start = time.time()
    for email, user in batch:
        if time.time() - start > 1 * 60:
            logging.warning("Time limit reached. Breaking out.")
            break
        body = html_tpl.replace('{{URL}}', quote_plus(email))
        body = body.replace('&#64;{{INSTAHANDLE}}', f'&#64;{user}')
        if send_email(sender, email, SUBJECT, body):
            sent_count += 1
            already_sent += 1
            write_file_to_s3(S3_BUCKET, LAST_RECEIVER_KEY, email)
            write_file_to_s3(S3_BUCKET, LAST_SENDER_KEY, sender)
            sender = next_sender(sender)
            time.sleep(random.uniform(1, 10))

    # persist counts
    write_file_to_s3(S3_BUCKET, TOTAL_SENT_KEY, str(already_sent))

    # completion check
    adjusted = already_sent + global_unsub
    logging.info(f"Adjusted count={adjusted}/{MAX_TOTAL_EMAILS}")
    if adjusted >= MAX_TOTAL_EMAILS:
        send_email(sender, SQS_NOTIFICATION_EMAIL, '✅ Email Batch Complete',
                   f'<p>All {MAX_TOTAL_EMAILS} emails accounted for (sent+unsub={adjusted}).</p>')
    else:
        send_email(sender, SQS_NOTIFICATION_EMAIL, '⏳ Lambda Re-Triggered',
                   f'<p>Sent {already_sent} + {global_unsub} unsub = {adjusted}/{MAX_TOTAL_EMAILS}.</p>')
        try:
            sqs.send_message(QueueUrl=SQS_QUEUE_URL, MessageBody='Trigger next batch')
        except ClientError as e:
            logging.error(f"Error triggering SQS: {e}")

    logging.info(f"Finished run: sent_run={sent_count}, overall_sent={already_sent}")

# Lambda entry
def lambda_handler(event, context):
    run_daily_emails(event, context)
    return {'statusCode': 200}
