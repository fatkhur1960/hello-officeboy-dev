#!/usr/bin/env python

import sys
import os
import json

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "libs", "apf-client-py"))

import apf

USER_NAME = "Zufar"
USER_EMAIL = "zufar@mail.com"
USER_PHONE = "+6285774931332"
USER_PASSHASH = "c4f79e6453e740fadae0e333a48888529f5cc10e7769491430fdcddff94d2f8f"

def collect_resp(resp):
    # print(resp.text)
    parsed = json.loads(resp.text)
    json_text = json.dumps(parsed, indent=4, sort_keys=False)
    return json_text

def ident_4(json_text):
    lines = json_text.split("\n")
    rv = []
    for line in lines:
        rv.append("        " + line)
    return "\n".join(rv)

def auth_authorize():
    global USER_NAME, USER_EMAIL, USER_PHONE, USER_PASSHASH
    return collect_resp(apf.authorize(USER_EMAIL, USER_PHONE, USER_PASSHASH))

def auth_get_key():
    return collect_resp(apf.get_key())

API_ENDPOINTS = {
    "group": {
        "name": "Authorization",
        "desc": "Endpoint berkaitan dengan otorisasi.",
        "endpoints": [
            {
                "path": "/auth/v1/authorize",
                "title": "Melakukan Otorisasi",
                "desc": "Biasanya digunakan untuk login.",
                "method": "POST",
                "func": auth_authorize
            }
        ]
    }
}

def main():
    global API_ENDPOINTS

    for k, group in API_ENDPOINTS.iteritems():
        print("## Group %s" % group["name"])
        print("")
        print("%s" % group["desc"])
        print("")
        for endp in group["endpoints"]:
            # (path, title, desc, method, func) = endp
            print("### %s [%s %s]" % (endp["title"], endp["method"], endp["path"]))
            print("")
            print(endp["desc"])
            print("")
            print(ident_4(endp["func"]()))
        

if __name__ == "__main__":
    main()
