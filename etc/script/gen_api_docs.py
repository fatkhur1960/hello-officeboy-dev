#!/usr/bin/env python

import sys
import os
import json
import re

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "libs", "apf-client-py"))

import apf


def get_path(path):
    return os.path.join(os.path.dirname(__file__), "..", "..", path)

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

def pretty_json_str(text):
    if not text or text == '':
        return ''
    try:
        parsed = json.loads(text)
        json_text = json.dumps(parsed, indent=4, sort_keys=False)
    except Exception as e:
        print("Cannot encode json: `%s`" % text)
    return json_text

TITLE_EXT_RE = re.compile(r"###(.*?) \[(GET|POST) (.*?)\]")

def load_doc(scope, in_path):
    global TITLE_EXT_RE
    docs = []
    endpoints_path = []
    line_num = 0

    # print("in_path: " + in_path)
    with open(in_path) as f:
        in_title = False
        in_group = False
        in_api_endpoint = False
        in_api_endpoint_parameters = False
        in_api_endpoint_request = False
        in_api_endpoint_response = False
        current_group = ""
        lines = f.readlines()
        for line in lines:
            line_num = line_num + 1
            # print("line: " + line)
            if line.startswith("HOST:"):
                docs.append({'elem': 'Host', 'value': line})
            elif line.startswith("# "):
                docs.append({'elem': 'MainTitle', 'value': line[2:].strip()})
                in_title = True
            elif in_title and not (line.startswith("#") or line.startswith("+") or line.startswith("   ")):
                # print("line: " + line)
                in_title = not (line.startswith("#") or line.startswith("+") or line.startswith("   "))
                if in_title and len(line.strip()) > 0:
                    if docs[-1]['elem'] != 'MainDesc':
                        docs.append({'elem': 'MainDesc', 'value': line})
                    else:
                        docs[-1]['value'] = (docs[-1]['value'] + '\n' + line).strip()
                    # continue
            elif line.startswith("## "):
                group_name = line[8:].strip()
                # print("group_name: " + group_name)
                docs.append({'elem': 'Group', 'group': group_name, 'title': group_name, 'desc': ""})
                current_group = group_name
                in_group = True

                in_title = False
                in_api_endpoint = False
                in_api_endpoint_request = False
                in_api_endpoint_response = False
                
            elif in_group:
                in_group = not line.startswith("## ")
                if in_group:
                    if line.startswith("### "):
                        in_api_endpoint = True
                        m = TITLE_EXT_RE.match(line)

                        title = m.group(1).strip()
                        method = m.group(2).strip()
                        path = m.group(3).strip()

                        if path in endpoints_path:
                            raise Exception("Endpoint terdeteksi duplikat: `%s` (line %d)" % (path, line_num))

                        rel_path = path.split('/')[-1]
                        method_name = title.lower().replace(' ', '_')
                        endpoints_path.append(path)

                        docs.append({'elem':"ApiEndpoint", 
                            "group": current_group, 
                            'path': path, 
                            'rel_path': rel_path, 
                            'method': method, 
                            'title': title, 
                            'desc': "",
                            'method_name': method_name,
                            'request_param': "",
                            'request_json': "",
                            'response_ok': ""})

                        in_api_endpoint_parameters = False
                        in_api_endpoint_request = False
                        in_api_endpoint_response = False
                        continue

                    elif in_api_endpoint:
                        # if line and len(line.strip()) > 0:
                        #     print(line)
                        in_api_endpoint = not line.startswith("##")
                        if in_api_endpoint:
                            if line.startswith('+ Request'):
                                in_api_endpoint_request = True
                                in_api_endpoint_parameters = False
                                in_api_endpoint_response = False
                                continue
                            elif line.startswith('+ Parameters'):
                                # print("::: " + line)
                                in_api_endpoint_parameters = True
                                in_api_endpoint_response = False
                                in_api_endpoint_request = False
                                continue
                            elif line.startswith('+ Response'):
                                # print("::: " + line)
                                in_api_endpoint_response = True
                                in_api_endpoint_request = False
                                in_api_endpoint_parameters = False
                                continue
                            elif in_api_endpoint_parameters:
                                in_api_endpoint_parameters = not line.startswith("+") and not line.startswith("#")
                                if in_api_endpoint_parameters:
                                    docs[-1]['request_param'] = (docs[-1]['request_param'] + '\n' + line).strip()
                            elif in_api_endpoint_request:
                                in_api_endpoint_request = not line.startswith("+") and not line.startswith("#")
                                if in_api_endpoint_request:
                                    docs[-1]['request_json'] = (docs[-1]['request_json'] + line).strip()
                            elif in_api_endpoint_response:
                                # print("line: " + line)
                                in_api_endpoint_response = not line.startswith("+") and not line.startswith("#")
                                if in_api_endpoint_response:
                                    # print(docs[-1])
                                    docs[-1]['response_ok'] = (docs[-1]['response_ok'] + '\n' + line).strip()
                            else:
                                docs[-1]['desc'] = (docs[-1]['desc'] + '\n' + line).strip()
                                
    return docs


def get_main_title(docs):
    a = filter(lambda a: a['elem'] == "MainTitle", docs)
    if a:
        return a[0]['value']
    return 'Untitled'

def get_main_desc(docs):
    a = filter(lambda a: a['elem'] == "MainDesc", docs)
    if a:
        return a[0]['value']
    return 'Rest API documentation'

def contain(item, docs):
    found = False
    for doc in docs:
        if doc['elem'] == item['elem']:
            if doc['elem'] == 'Group':
                found = doc['title'] == item['title']
            elif doc['elem'] == 'ApiEndpoint':
                found = doc['path'] == item['path']
        if found:
            return True
    return False

def merge_doc(orig_docs, other_docs):

    for other in other_docs:
        if not contain(other, orig_docs):
            orig_docs.append(other)

    for orig in orig_docs:
        for other in other_docs:
            if orig['elem'] == other['elem']:
                if orig['elem'] == 'ApiEndpoint' and orig['path'] == other['path']:
                    orig['title'] = other['title']
                    orig['desc'] = other['desc']
                    orig['method'] = other['method']
                    orig['method_name'] = other['method_name']
            
    return

def gen_doc(scope, in_path, out_path):
    parsed_docs = load_doc(scope, out_path)
    # print(json.dumps(parsed_docs, indent=4, sort_keys=False))
    # return

    with open(out_path + ".tmp~", "w") as fout:
        fout.write("FORMAT: 1A\n\n")
        
        # @TODO(robin): 2 lines di bawah ini dibuat agar configurable (tidak hardcoded)
        fout.write("# %s\n\n" % get_main_title(parsed_docs))
        fout.write("%s\n" % get_main_desc(parsed_docs))

        new_docs = []

        with open(in_path) as f:
            lines = f.readlines()
            for line in lines:
                j = json.loads(line)
                new_docs.append(j)

        merge_doc(parsed_docs, new_docs)

        def sorter(a, b):
            if a.has_key('group') and b.has_key('group'):
                return cmp(a['group'], b['group'])
            return 0

        updated_docs = sorted(parsed_docs, cmp=sorter)
        groups = filter(lambda a: a["elem"] == "Group", updated_docs)
        endpoints = sorted(filter(lambda a: a["elem"] == "ApiEndpoint", updated_docs), lambda a,b: cmp(a['method_name'], b['method_name']))

        for group in groups:
            process_line(group, fout)
            for endpoint in endpoints:
                if endpoint['group'] == group['group']:
                    process_line(endpoint, fout)
    
    os.rename(out_path + '.tmp~', out_path)

BP_PARAM_RE = re.compile(r"\+ (.*?):\s*([0-9]*).*?\-s*(.*)")

def parse_query_params(param_str):
    rv = []
    for line in param_str.split('\n'):
        line = line.strip()
        rs = BP_PARAM_RE.match(line)
        if rs:
            # print(rs.groups())
            key = rs.group(1).strip()
            value = rs.group(2).strip()
            desc = rs.group(3).strip()
            rv.append(dict(key=key, value=value, description=desc))
    return rv


def gen_postman(api_scope, input_path, out_path):
    parsed_docs = load_doc(api_scope, input_path)
    d = {
        "info": {
            "_postman_id": "cb12386d-1896-449c-93e6-d6da8ff6e800",
            "name": "??",
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
	    },
        "item": []
    }

    for m in parsed_docs:
        if m['elem'] == "MainTitle":
            d['info']['name'] = m['value'] + " (" + api_scope + ")"
        elif m['elem'] == "Group":
            d['item'].append({'name': m['title'], 'item': []})
        elif m['elem'] == "ApiEndpoint":
            if type(d['item'][-1]['item']) is not list:
                raise Exception("prev element not `Group`")
            
            query_params = parse_query_params(m['request_param'])

            d['item'][-1]['item'].append({
                'name': m['title'],
                'request': {
                    'method': m['method'],
                    'header': [
                        {
                            "key": "Content-Type",
                            "value": "application/json"
                        },
                        {
                            "key": "Accept",
                            "value": 'application/json',
                            'description': 'Request JSON'
                        }
                    ],
                    'body': {
                        'mode': "raw",
                        'raw': pretty_json_str(m['request_json'])
                    },
                    "url": {
                        "raw": "{{base_url}}/%s" % m['path'],
                        "host": ["{{base_url}}"],
                        "path": list(filter(lambda a: len(a.strip()) > 0, m["path"].split("/"))),
                        "query": query_params
                    }
                },
                'response': [
                    {
                        'header': [
                            {
                                "key": "Content-Type",
                                "value": "application/json"
                            }
                        ],
                        'status': '200 OK',
                        'code': 200,
                        'body': pretty_json_str(m['response_ok'])
                    }
                ]
            })

    # print(json.dumps(d, indent=4, sort_keys=False))

    with open(out_path, "w") as fout:
        fout.write(json.dumps(d, indent=4, sort_keys=False))


def process_line(j, fout):
    if j["elem"] == "Group":
        fout.write("## Group %s\n" % j["title"].strip())
        if j["desc"] and j["desc"] != "":
            fout.write("\n%s\n\n" % j["desc"].strip())
        else:
            fout.write("\n")
    elif j["elem"] == "ApiEndpoint":
        title = j['title']
        if not title or title == "":
            # s = j['path'].split('/')
            # title = s[-1].title()
            title = j['method_name'].replace('_', ' ').title()
        fout.write("### %s [%s %s]\n\n" % (title, j['method'], j['path']))
        fout.write("%s\n\n" % j['desc'])
        if j['request_param'] and j['request_param'] != "":
            fout.write("+ Parameters\n\n")
            request_param = j['request_param']
            
            fout.write("    %s\n\n" % request_param)

        elif j['request_json'] and j['request_json'] != "":
            fout.write("+ Request JSON (application/json)\n\n")
            
            try:
                fout.write("%s\n\n" % json_print(j['request_json'].strip()))
            except Exception as e:
                print("e: %s" % e)
                raise Exception("Format json tidak valid untuk request API `%s`: `%s`" % (j['path'], j['request_json']))

        fout.write("+ Response 200 (application/json)\n\n")
        if j['response_ok'] and j['response_ok'] != "":
            try:
                fout.write("%s\n\n" % json_print(j['response_ok']))
            except Exception as e:
                raise Exception("Format json tidak valid untuk response API `%s`" % j['path'])
        else:
            fout.write("%s\n\n" % ident_4("{}"))

def main():
    public_input_path = get_path("api-docs/public-endpoints.raw.txt")
    private_input_path = get_path("api-docs/private-endpoints.raw.txt")
    
    public_blp = get_path("api-docs/public-api.md")
    private_blp = get_path("api-docs/private-api.md")

    
    gen_doc("public", public_input_path, public_blp)
    gen_doc("private", private_input_path, private_blp)

    gen_postman("public", public_blp, get_path("target/public-api.postman"))
    gen_postman("private", private_blp, get_path("target/private-api.postman"))


if __name__ == "__main__":
    main()
