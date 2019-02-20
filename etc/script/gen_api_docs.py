#!/usr/bin/env python

import sys
import os
import json

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "libs", "apf-client-py"))

import apf


def get_path(path):
    return os.path.join(os.path.dirname(__file__), "..", "..", path)

def gen_doc(scope, in_path, out_path):
    with open(out_path, "w") as fout:
        fout.write("FORMAT: 1A\n\n")
        
        # @TODO(robin): 2 lines di bawah ini dibuat agar configurable (tidak hardcoded)
        fout.write("# APF rest API documentation\n\n")
        fout.write("Dokumentasi rest API\n\n")

        with open(in_path) as f:
            lines = f.readlines()
            for line in lines:
                process_line(line, fout)

def ident_4(json_text):
    lines = json_text.split("\n")
    rv = []
    for line in lines:
        rv.append("        " + line)
    return "\n".join(rv)


def json_print(text):
    parsed = json.loads(text)
    json_text = json.dumps(parsed, indent=4, sort_keys=False)
    return ident_4(json_text)

def process_line(line, fout):
    j = json.loads(line)
    if j["elem"] == "Group":
        fout.write("## Group %s\n" % j["title"].strip())
        if j["desc"] and j["desc"] != "":
            fout.write("\n%s\n\n" % j["desc"].strip())
        else:
            fout.write("\n");
    elif j["elem"] == "ApiEndpoint":
        title = j['title']
        if not title or title == "":
            s = j['path'].split('/')
            title = s[-1].title()
        fout.write("### %s [%s %s]\n\n" % (title, j['method'], j['path']))
        fout.write("%s\n\n" % j['desc'])
        if j['request_json'] and j['request_json'] != "":
            fout.write("+ Request JSON (application/json)\n\n")
            fout.write("%s\n" % json_print(j['request_json']))
        fout.write("+ Response 200 (application/json)\n\n")
        if j['response_ok'] and j['response_ok'] != "":
            fout.write("%s\n\n" % json_print(j['response_ok']))
        else:
            fout.write("%s\n\n" % ident_4("{}"))

def main():
    public_input_path = get_path("api-docs/public-endpoints.raw.txt")
    private_file_name = get_path("api-docs/private-endpoints.raw.txt")
    
    public_output_path = get_path("api-docs/public-api-gen.md")
    private_output_path = get_path("api-docs/private-api-gen.md")

    
    gen_doc("public", public_input_path, public_output_path)
    



if __name__ == "__main__":
    main()
