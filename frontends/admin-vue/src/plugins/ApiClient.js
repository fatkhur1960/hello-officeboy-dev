import _axios from 'axios';


export default class ApiClient { 
  constructor(publicApiBaseUrl, privateApiBaseUrl) {
    this.headers = {'Content-Type':'application/json'};

    this.publicApi = _axios.create({
        baseURL: publicApiBaseUrl,
        timeout: 10000,
        headers: this.getHeaders()
    });
    this.privateApi = _axios.create({
        baseURL: privateApiBaseUrl,
        timeout: 10000,
        headers: this.getHeaders()
      })
    }

    getHeaders(){
      return this.headers;
    }
}




