import ApiClient from "./ApiClient";
import crypto from './apf-crypto';

export default class Apf {
  static install(Vue) {
    var api = new ApiClient("http://localhost:8081/api/payment/v1", 
      "http://localhost:8082/api/payment/v1");

    updateSession();

    function updateSession(){
      var token = Vue.prototype.$session.get("token");
      api.publicApi.defaults.headers["X-Access-Token"] = token;
    }

    Vue.prototype.$apf = {
      login(email, phone, password) {
        var passhash = crypto.getPasshash(password);
        console.log("passhash: " + passhash);
        var emailOrPhone = email ? email : phone;
        var data = {
          "email": emailOrPhone,
          "phone": phone,
          "passhash": passhash
        };
        return api.publicApi.post("/authorize", data)
          .then((resp) => {
            if (resp.data.token){
              Vue.prototype.$session.set("token", resp.data.token);
              updateSession(resp.data.token);
            }
            return resp;
          })
          ;
      },
      unauthorize() {
        Vue.prototype.$session.remove("token");
        updateSession();
        return api.publicApi.post("/unauthorize", {});
      },
      isLoggedIn(cb){
        this.getMeInfo().then((resp) => {
          if (resp.status != 200 || (resp.data.status == "error" && resp.data.code != 0) ){
            cb(false)
          }else{
            cb(true)
          }
        }).catch((_e) => cb(false))
      },
      getMeInfo(){
        return api.publicApi.get("/me/info");
      },
      api(){
        return api;
      }
    }
  }
  static version = ""
}


