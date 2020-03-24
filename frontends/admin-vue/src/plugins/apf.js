import { ApiClient, Crypto } from "../../../../libs/apf-client-js";
// import crypto from "../../../../libs/apf-client-js/apf-crypto";

import * as protos from '../proto/stubs';

export default class Apf {
  static install(Vue) {

    var api;

    if (Vue.config.runMode == "prod") {
      api = new ApiClient("http://localhost:8080/api",
        "http://localhost:9090/api");
    } else if (Vue.config.runMode == "dev") {
      api = new ApiClient("http://localhost:8080/api",
        "http://localhost:9090/api");
    } else if (Vue.config.runMode == "mock") {
      api = new ApiClient("http://private-b1a4a4-anvie.apiary-mock.com/api",
        "http://private-b1a4a4-anvie.apiary-mock.com/api");
    } else {
      throw "Unknown mode: " + Vue.config.runMode
    }

    updateSession();

    // var $session = Vue.prototype.$session;
    function session() {
      return Vue.prototype.$session;
    }

    function updateSession() {
      var token = session().get("token");
      api.publicApi.defaults.headers["X-Access-Token"] = token;
      api.privateApi.defaults.headers["X-Access-Token"] = token;
    }

    Vue.prototype.$apf = {
      crypto() {
        return Crypto;
      },
      login(email, phone, password) {
        var passhash = Crypto.getPasshash(password);
        console.log("passhash: " + passhash);
        var emailOrPhone = email ? email : phone;
        var data = {
          "email": emailOrPhone,
          "phone": phone,
          "passhash": passhash
        };
        return api.publicApi.post("/auth/v1/authorize", data)
          .then((resp) => {
            if (resp.data.token) {
              session().set("token", resp.data.token);
              updateSession(resp.data.token);
              this.loadAccountKey();
            }
            return resp;
          });
      },
      unauthorize() {
        console.log("unauthorize");
        session().remove("token");
        updateSession();
        return api.publicApi.post("/api/auth/v1/unauthorize", {});
      },
      isLoggedIn(cb) {
        this.getMeInfo().then((resp) => {
          if (resp.status != 200 || (resp.data.status == "error" && resp.data.code != 0)) {
            cb(false)
          } else {
            cb(true)
          }
        }).catch((_e) => cb(false))
      },
      getMeInfo() {
        return api.publicApi.get("/payment/v1/me/info");
      },

      // Fetch current account key-pair.
      loadAccountKey() {
        return api.publicApi.get("/auth/v1/get_key")
          .then((resp) => {
            console.log("account key loaded.");
            session().set("pk", resp.data.result.pub_key);
            session().set("sk", resp.data.result.secret_key);
          }).catch(_e => {
            // this.$notify({
            //   group: "default",
            //   type: "error",
            //   title: "Error",
            //   text: "Cannot load keys"
            // });
          });
      },

      // Mendapatkan current user key pair dari local session.
      getKeys() {
        var pk = session().get("pk");
        var sk = session().get("sk");
        return {
          pubKey: Buffer.from(pk, 'hex'),
          secretKey: Buffer.from(sk, 'hex'),
        }
      },
      creditAccountBalance(accountId, amount) {
        var credit = new protos.payment.Credit({
          account: accountId,
          amount: parseFloat(amount),
          timestamp: this.now(),
          seed: this.generateSeed()
        });

        var buffer = protos.payment.Credit.encode(credit).finish();
        let keys = this.getKeys();
        var signature = Crypto.sign(buffer, keys.pubKey, keys.secretKey);

        var data = {
          body: protos.payment.Credit.toObject(credit),
          signature: signature
        };
        return api.privateApi.post("/payment/v1/credit", data);
      },
      generateSeed() {
        return Math.floor(Math.random() * 1000000000);
      },
      now() {
        return new Date().getTime();
      },
      api() {
        return api;
      }
    }
  }
  static version = ""
}


