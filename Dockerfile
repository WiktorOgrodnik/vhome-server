FROM postgres:15

RUN localedef -i pl_PL -c -f UTF-8 -A /usr/share/locale/locale.alias pl_PL.UTF-8
ENV LANG pl_PL.utf8

RUN apt update \
      && apt install -y --no-install-recommends vim git python3 ssh-client python3-pip python3-setuptools python3-dev musl-dev
RUN useradd --create-home --shell /bin/bash vuser

USER vuser
WORKDIR /home/vuser

ENV POSTGRES_PASSWORD=secret_pass
ENV PGDATA=/var/lib/postgresql/data/pgdata
COPY ./init-vuser-db.sh /docker-entrypoint-initdb.d/init-vuser-db.sh

