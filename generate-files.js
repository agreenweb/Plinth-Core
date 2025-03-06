
const fs = require("fs");
const path = require("path");

const assetsDir = path.join(__dirname, "dist/assets");
const outputFile = path.join(__dirname, "dist/files.json");

// Get all `.js` and `.css` files
fs.readdir(assetsDir, (err, files) => {
	if (err) {
		console.error("Error reading assets directory:", err);
		return;
	}

	const scripts = files.filter(file => file.endsWith(".js"));
	const styles = files.filter(file => file.endsWith(".css"));

	// Write JSON file with both scripts and styles
	fs.writeFileSync(outputFile, JSON.stringify({ scripts, styles }, null, 2));
	console.log("Generated files.json with", scripts.length, "scripts and", styles.length, "styles");
});
