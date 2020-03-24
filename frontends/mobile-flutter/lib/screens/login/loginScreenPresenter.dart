import 'dart:convert';
import 'package:hello_flutter/data/api_client.dart';
import 'package:hello_flutter/utils/crypt.dart';
import 'package:hello_flutter/utils/env.dart';
import 'package:hello_flutter/models/login.dart';

abstract class LoginScreenContract {
  void onLoginSuccess(Login login);
  void onLoginError(String errorTxt);
}

class LoginScreenPresenter {
  LoginScreenContract _view;
  LoginScreenPresenter(this._view);
  final ApiClient api = new ApiClient();
  final Crypt crypt = new Crypt();
  final JsonDecoder _decoder = new JsonDecoder();

  doLogin(String username, String password) {
    print("in doLogin()");
    api.publicApi('').post('/auth/v1/authorize', body: {
      "email": username,
      "phone": '',
      "passhash": crypt.getPassHash(password)
    }).then((String response) {
      var res = _decoder.convert(response);
      if(res["code"] == 0) {
        print("[i] Login success!");
        _view.onLoginSuccess(new Login.map(res));
      } else {
        print("[i] Login failed!");
        _view.onLoginError("[i] Login failed!");
      }
    });
  }
}