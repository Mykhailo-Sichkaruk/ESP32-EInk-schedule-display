// import https from "node:https";
import fs from "node:fs/promises";
import http from "node:http";
import IcalExpander from "ical-expander";
import { faker } from "@faker-js/faker";

process.env.TZ = "Europe/Bratislaba";

let scheduleCounter = 2961;
let errorCounter = 98;

function addDate({ unixTimeDate, days = 0, hours = 0, minutes = 0 }) {
  let date = new Date(unixTimeDate);
  date.setDate(date.getDate() + days);
  date.setHours(date.getHours() + hours);
  date.setMinutes(date.getMinutes() + minutes);
  return date.getTime();
}

function addDate2({ unixTimeDate, unixDeltaTime }) {
  let date = new Date(unixTimeDate);
  date.setDate(date.getDate() + Math.floor(unixDeltaTime / 1 / 60 / 60 / 24));
  date.setHours(date.getHours() + Math.floor(unixDeltaTime / 1 / 60 / 60));
  date.setMinutes(date.getMinutes() + Math.floor(unixDeltaTime / 1 / 60));
  return date.getTime();
}

function createTime(num, now, step = 60) {
  return {
    label: faker.person.fullName().slice(0, 10),
    start_unix: addDate({ unixTimeDate: now, days: 0, hours: 0, minutes: num * step }),
    end_unix: addDate({ unixTimeDate: now, days: 0, hours: 0, minutes: (num + 1) * step }),
  }
}

function gen2(num, now) {
  return Array(num).keys().map(
    (i) => {
      return {
        label: faker.person.fullName().slice(0, 10),
        start_unix: addDate2({ unixTimeDate: now, unixDeltaTime: i * 15 * 60 }),
        end_unix: addDate2({ unixTimeDate: now, unixDeltaTime: (i + 1) * 15 * 60 }),
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
  const now = Date.now();
  return {
    widgets: {
      schedule: {
        events: gen(75, now),
      }
    },
    system: {
      server_time_unix: Date.now(),
    },
  };
};


const getCal = async (now, days) => {
  const response = await fetch("https://calendar.google.com/calendar/ical/c_fc7301a05cf616d1de9b9fcd497f83026c333f0f5aaa601f24295d1ddc97e2f1%40group.calendar.google.com/private-9b61d8c6e520d380428debcfd3e5a290/basic.ics");
  const body = await response.text();

  const cal = new IcalExpander({ ics: body, maxIterations: 1000 });
  const nowDate = new Date(now);
  nowDate.setHours(0, 0, 0);
  const nextDate = new Date(now);
  nextDate.setDate(nowDate.getDate() + days);
  nextDate.setHours(23, 59, 59);
  const calLastEvents = cal.between(nowDate, nextDate);

  const events = calLastEvents.events.map(event => ({ start_unix: event.startDate.toJSDate().getTime(), end_unix: event.endDate.toJSDate().getTime(), label: event.summary ?? "Bez nazvu" }));

  return {
    widgets: {
      schedule: {
        events,
      }
    },
    system: {
      server_time_unix: Date.now(),
    },
  };
}

const requestListener = async (req, res) => {
  console.log({ host: req.headers['host'], time: new Date().toString() });
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
    errorCounter++;
    console.dir({ errorCounter });
    console.error("ESP Reports error", req.body);
  } else {
    const events = await getCal(Date.now(), 2).catch((e) => { 
      console.error("Error fetching calendar:", e); 
      return { widgets: { schedule: { events: [] } }, system: { server_time_unix: Date.now() } }; 
    });
    res.writeHead(200, { "Content-Type": "application/json" });
    // const result = getResult();
    res.end(JSON.stringify(events));
    scheduleCounter++;
    console.dir({ scheduleCounter });
    await fs.writeFile("counter.txt", Buffer.from(String(scheduleCounter)));
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

