import ApiClient from "./ApiClient";
import crypto from './apf-crypto';

export default class Apf {
  static install(Vue) {
    var api = new ApiClient("http://localhost:8081/api/payment/v1", 
      "http://localhost:8082/api/payment/v1");

    var token = Vue.prototype.$session.get("token");
    api.publicApi.defaults.headers["X-Access-Token"] = token;

    Vue.prototype.$apf = {
      login(email, phone, password) {
        var passhash = crypto.get_passhash(password);
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
              this.updateToken(resp.data.token);
            }
            return resp;
          })
          ;
      },
      logout() {
        return api.publicApi.post("/unauthorize", {});
      },
      getMeInfo(){
        return api.publicApi.get("/me/info");
      },
      api(){
        return api;
      },
      updateToken(token){
        Vue.prototype.$session.set("token", token);
        api.publicApi.defaults.headers["X-Access-Token"] = token;
      }
    }
  }
  static version = ""
}


