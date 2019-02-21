import 'dart:io';
import 'package:hello_flutter/utils/network_util.dart';
import 'package:hello_flutter/utils/env.dart';

class ApiClient {

  publicApi(String token) {
    return new NetworkUtil(baseurl: publicURL, token: token);
  }

  privateApi(String token) {
    return new NetworkUtil(baseurl: privateURL, token: token);
  }

}