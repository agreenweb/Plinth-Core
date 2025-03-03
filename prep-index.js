const fs = require("fs");
const path = require("path");

// Get the build mode from command line arguments
const mode = process.argv[2]; // Expected: "vite" or "trunk"

if (!mode || (mode !== "vite" && mode !== "trunk")) {
	console.error('Usage: node prep-index.js [vite|trunk]');
	process.exit(1);
}

const srcFile = "./src/web/src.html";
const outputFile = "./index.html";
const tsxDir = "./src/web/ts/";

try {
	// Read the source HTML file
	let content = fs.readFileSync(srcFile, "utf8");

	if (mode === "vite") {
		// Find all `.tsx` files in `tsxDir`
		const tsxFiles = fs.readdirSync(tsxDir)
			.filter(file => file.endsWith(".tsx"))
			.map(file => `<script type="module" src="./src/web/ts/${file}"></script>`)
			.join("\n");

		// Replace the first line containing "VITE" with the generated script tags
		content = content.replace(/^.*VITE.*$/m, tsxFiles);

		console.log("✅ Generated index.html for Vite.");
	} else if (mode === "trunk") {
		// Remove the line containing "VITE"
		content = content.split("\n").filter(line => !line.includes("VITE")).join("\n");

		console.log("✅ Generated index.html for Trunk.");
	}

	// Write the modified content to `index.html`
	fs.writeFileSync(outputFile, content, "utf8");
} catch (error) {
	console.error("❌ Error processing index.html:", error);
	process.exit(1);
}
