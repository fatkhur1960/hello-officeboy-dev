

const _axios = require("axios");

export default {
    public: _axios.create({
        baseURL: "http://localhost:8081/api/payment/v1",
        timeout: 1000,
    }),
    private: _axios.create({
        baseURL: "http://localhost:8082/api/payment/v1",
        timeout: 1000,
      }),
};

