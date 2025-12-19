// import https from "node:https";
import http from "node:http";
import fs from "node:fs";
import { faker } from "@faker-js/faker";

const PORT = 8080;

function addDate({ unixTimeDate, days = 0, hours = 0, minutes = 0 }) {
  let date = new Date(unixTimeDate);
  date.setDate(date.getDate() + days);
  date.setHours(date.getHours() + hours);
  date.setMinutes(date.getMinutes() + minutes);
  return date.getTime();
} 

const getResult = () => { 
  const now = 1766179737817;
  return {
    server_time: Date.now(),
    items: [
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 0, hours: -5, minutes: -30 }), end: addDate({ unixTimeDate: now, days: 0, hours: -4, minutes: -30 })},
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 0, hours: -4, minutes: -30 }), end: addDate({ unixTimeDate: now, days: 0, hours: -3, minutes: -30 })},
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 0, hours: 0, minutes: 0 }), end: addDate({ unixTimeDate: now, days: 0, hours: 1, minutes: 0 })}, 
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 0, hours: 1, minutes: 0 }), end: addDate({ unixTimeDate: now, days: 0, hours: 2, minutes: 0 })},
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 0, hours: 2, minutes: 0 }), end: addDate({ unixTimeDate: now, days: 0, hours: 3, minutes: 0 })},
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 1, hours: 0, minutes: 0 }), end: addDate({ unixTimeDate: now, days: 1, hours: 2, minutes: 0 })},
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 2, hours: 1, minutes: 0 }), end: addDate({ unixTimeDate: now, days: 2, hours: 2, minutes: 0 })},
      { label: faker.person.fullName(), start: addDate({ unixTimeDate: now, days: 1, hours: -10, minutes: 0 }), end: addDate({ unixTimeDate: now, days: 1, hours: -2, minutes: 0 })},
    ], 
  };
};

const requestListener = (req, res) => {
  console.log({ headers: req.rawHeaders, time: new Date().toISOString() });
  res.writeHead(200, { "Content-Type": "application/json" });
  res.end(JSON.stringify(getResult()));
};

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
