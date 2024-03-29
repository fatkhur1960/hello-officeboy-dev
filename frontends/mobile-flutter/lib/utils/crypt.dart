import 'dart:convert';
import 'package:crypto/crypto.dart';

class Crypt {
  
  String getPassHash(String input) {
    var bytes = utf8.encode(input);
    var hash = sha256.convert(bytes);
    for (var i = 0; i < 9; i++) {
      hash = this.makeSha256(hash);
    }
    return hash.toString();
  }

  Digest makeSha256(Digest input) {
    var hash = sha256.convert(input.bytes);
    return hash;
  }

}