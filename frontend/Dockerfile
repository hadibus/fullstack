FROM node:17
WORKDIR /usr/src/frontend
COPY ./ .
RUN npm install && npm run build
CMD ["npm","start"]
