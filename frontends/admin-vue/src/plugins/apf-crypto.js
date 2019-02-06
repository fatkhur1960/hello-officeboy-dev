
import crypto from "crypto";


export default {
  // Mendapatkan passhash dari plain password.
  getPasshash(plainPassword){
    var bytes = Buffer.from(plainPassword, 'utf-8');
    var hash = this.sha256(bytes);
    Array(9).fill().map((_, _i) => {
      hash = this.sha256(hash);
    });
    return util.toHexString(hash);
  },
  // Kalkulasikan hash sha256 pada data.
  sha256(data, output){
    return crypto.createHash("sha256").update(data).digest(output);
  }
}

var util = {
  // Konversi bytes ke hexa string.
  toHexString(byteArray) {
    return Array.from(byteArray, function(byte) {
      return ('0' + (byte & 0xFF).toString(16)).slice(-2);
    }).join('')
  }
}