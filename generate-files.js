
const fs = require("fs");
const path = require("path");

const jsDir = path.join(__dirname, "dist/js");
const outputFile = path.join(__dirname, "dist/files.json");

// Get all `.js` files
fs.readdir(jsDir, (err, files) => {
	if (err) {
		console.error("Error reading JS directory:", err);
		return;
	}

	const scripts = files.filter(file => file.endsWith(".js"));

	// Write JSON file
	fs.writeFileSync(outputFile, JSON.stringify({ scripts }, null, 2));
	console.log("Generated files.json with", scripts.length, "files");
});
