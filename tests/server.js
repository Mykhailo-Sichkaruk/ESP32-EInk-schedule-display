// import https from "node:https";
// import fs from "node:fs";
import http from "node:http";
import { faker } from "@faker-js/faker";

function addDate({ unixTimeDate, days = 0, hours = 0, minutes = 0 }) {
  let date = new Date(unixTimeDate);
  date.setDate(date.getDate() + days);
  date.setHours(date.getHours() + hours);
  date.setMinutes(date.getMinutes() + minutes);
  return date.getTime();
} 

function addDate2({ unixTimeDate, unixDeltaTime }) {
  let date = new Date(unixTimeDate);
  date.setDate(date.getDate() + Math.floor(unixDeltaTime/1/60/60/24));
  date.setHours(date.getHours() + Math.floor(unixDeltaTime/1/60/60));
  date.setMinutes(date.getMinutes() + Math.floor(unixDeltaTime/1/60));
  return date.getTime();
} 

function createTime(num, now, step = 60) {
  return {
    label: faker.person.fullName().slice(0, 10),
    start: addDate({ unixTimeDate: now, days: 0, hours: 0, minutes: num * step }),
    end: addDate({ unixTimeDate: now, days: 0, hours: 0, minutes: (num + 1) * step }),
  }
}

function  gen2(num, now) {
  return Array(num).keys().map(
    (i) => {
      return {
        label: faker.person.fullName().slice(0, 10),
        start: addDate2({unixTimeDate: now, unixDeltaTime: i*15*60}),
        end: addDate2({unixTimeDate: now, unixDeltaTime: (i+1)*15*60}),
      };
    }
  )
}

function gen(num, now) {
  let arr = [];
  for (let a = 0; a < num; a++) { 
    arr.push(createTime(a, now));
  }

  return arr;
}

const getResult = () => { 
  // const now = 1766179737817;
  const now = 1767041427334;
  return {
    server_time: Date.now(),//  + 21 * 60 * 1000,
    items: gen(75, now), 
    // items: [...gen2(150, now)],
  };
};

const requestListener = async (req, res) => {
  console.log({ host: req.headers['host'], time: new Date().toISOString() });
  if (req.url === "/error") {
    const body = await new Promise((resolve) => {
      let data = "";
      req.on("data", (chunk) => {
        data += chunk;
      });
      req.on("end", () => {
        resolve(data);
      });
    });
    req.body = JSON.parse(body);
    res.writeHead(200);
    res.end();
    console.error("ESP Reports error", req.body);
  } else {
    res.writeHead(200, { "Content-Type": "application/json" });
    const result = getResult();
    res.end(JSON.stringify(result));
    console.log(result);
  }
};

const PORT = 8080;
// const options = {
//   key: fs.readFileSync("./certs/localhost-key.pem"),
//   cert: fs.readFileSync("./certs/localhost.pem"),
// };
//
// https.createServer(options, requestListener).listen(PORT, "0.0.0.0", () => {
//   console.log(`Server is running on https://0.0.0.0:${PORT}`);
// });
//
http.createServer(requestListener).listen(PORT, "0.0.0.0", () => {
  console.log(`Server is running on http://0.0.0.0:${PORT}`);
});
