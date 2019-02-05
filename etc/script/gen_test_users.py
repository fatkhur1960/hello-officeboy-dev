#!/usr/bin/env python

import requests
import json

BASE_URL = "http://localhost:8081/api/payment/v1"
HEADERS = {'Content-Type': 'application/json'}

def api_url(path):
    global BASE_URL
    return "%s/%s" % (BASE_URL, path)

def api_post(path, data):
    global HEADERS
    return requests.post(api_url(path), data=json.dumps(data), headers=HEADERS)

def register_account(full_name, email, phone_num):
    rv = api_post("account/register", {
        "full_name": full_name,
        "email": email,
        "phone_num": phone_num
    })
    # print(rv)
    rv = json.loads(rv.text)
    if rv.get("status") and rv['status'] == "error":
        print("ERROR: %s" % rv["description"])
        return {}
    return rv["result"]

def activate_account(token, password):
    rv = api_post("account/activate", {
        "token": token,
        "password": password
    })
    return rv


def main():

    target_accounts = [
        ["Zufar", "zufar@ansvia.com", "+628123123"],
        ["Akmal", "akmal@ansvia.com", "+628123124"],
        ["Anto", "anto@ansvia.com", "+628123125"],
        ["Hanky", "hanky@ansvia.com", "+628123126"],
        ["Andrie", "andrie@ansvia.com", "+628123127"],
        ["Ubai", "ubai@ansvia.com", "+628123128"],
    ]

    tokens = []
    for acc in target_accounts:
        tokens.append(register_account(acc[0], acc[1], acc[2]))

    print(tokens)

    for token in tokens:
        if type(token) == unicode:
            if activate_account(token, "123").status_code != 200:
                print("cannot activate account with token %s", token)
        

if __name__ == "__main__":
    main()
