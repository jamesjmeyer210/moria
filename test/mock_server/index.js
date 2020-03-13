'use strict';

const express = require('express');
const bodyParser = require('body-parser');
const fs = require('fs');
const path = require('path');

const runServer = (config) => {
	// Initialize the expressjs server app.
	// Use the json body parser so we can extract json objects from post requests.
	const app = express();
	app.use(bodyParser.json());

	// Define a POST request route.
	// Wrap the functionality of this route in the setTimeout function so we can
	//	apply a throttle.
	app.post("/api/user", (request, response) => {
		console.log("POST /api/user");
		console.log(`Headers: ${JSON.stringify(request.headers)}`);		

		setTimeout(() => {
			response.status(200).send(JSON.stringify(request.body));
		}, config.throttle);
	});

	app.get("/api/user", (request, response) => {
		console.log("GET /api/user");		
		const user = {
			id: 0,
			name: 'Alice',
			email: 'alice@email.com',
		};

		const body = JSON.stringify(user);

		setTimeout(() => {
			response.status(200).set({
				'Content-Type':'application/json',
				'Content-Length':`${body.length}`
			}).send(body);
		}, config.throttle);
	});

	app.get("/api/always/404", (request, response) => {
		console.log("GET /api/always/404");
		setTimeout(() => {
			response.status(404).send();
		}, config.throttle);
	});

	app.get("/api/always/401", (request, response) => {
		console.log("GET /api/always/404");
		setTimeout(() => {
			response.status(401).send();
		}, config.throttle);
	});

	// Run the server on whatever port is defined by the config file.
	// Presently, this script only runs an http server.
	const server = app.listen(config.port, () => {
	   	console.log(`Listening on ${config.port}`);
	});

};

// Begin index.js
fs.readFile(path.join(process.cwd(), "config.json"), {encoding: 'utf-8'}, (error, data) => {
	if(error){
		console.log(error);
	}
	else{
		// Config file successfully read in.
		// Parse the config file into an object and run the server.
		console.log(data);
		const config = JSON.parse(data);
		runServer(config);
	}
});
