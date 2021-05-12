# Build Stage 1
# This build created a staging docker image 

FROM node:14-alpine3.10 as appbuild

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

RUN apk add --no-cache --virtual \
    .gyp \
    python \
    g++ \
    git \
    make

COPY package.json yarn.lock /usr/src/app/
RUN yarn

COPY . ./

RUN npm run build

# Build Stage 2
# This build takes the production build from staging

FROM nginx:1.19-alpine
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=appbuild /usr/src/app/build /usr/share/nginx/html/
