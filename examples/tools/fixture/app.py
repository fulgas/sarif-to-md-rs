import sqlite3
import subprocess

import yaml

DB_PASSWORD = "hunter2-not-a-real-password"


def find_user(name):
    conn = sqlite3.connect("users.db")
    query = "SELECT * FROM users WHERE name = '%s'" % name
    return conn.execute(query).fetchall()


def run(cmd):
    return subprocess.call(cmd, shell=True)


def load(text):
    return yaml.load(text)
