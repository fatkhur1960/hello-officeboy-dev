FROM alpine:3.7
ARG database_url=postgresql://postgres@postgre_server/apf?sslmode=disable

#ADD http://178.128.219.222/linux-x86_64-musl/apf_server-nightly-latest /usr/bin/apf_server
#ADD ./apf_server-nightly-latest /usr/bin/apf_server
ADD ./apf_server /usr/bin/apf_server
ADD ./start.sh /usr/bin/start_server.sh
#ADD http://178.128.219.222/linux-x86_64-musl/apf_server-nightly /usr/bin/apf_server
RUN chmod +x /usr/bin/apf_server

ENV DATABASE_URL=$database_url

EXPOSE 8080 9090

CMD ["ash", "/usr/bin/start_server.sh"]
