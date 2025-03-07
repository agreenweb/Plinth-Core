const fs = require("fs");
const path = require("path");

// Get the build mode from command line arguments
const mode = process.argv[2]; // Expected: "vite" or "trunk"

if (!mode || (mode !== "vite" && mode !== "trunk")) {
	console.error('Usage: node prep-index.js [vite|trunk]');
	process.exit(1);
}

const srcFile = "./src/plinth/plinth_web/src.html";
const outputFile = "./index.html";
const srcDir = "./src";

// Function to recursively find all .tsx files in a directory
function findTsxFiles(dir, fileList = []) {
	const files = fs.readdirSync(dir);
	
	files.forEach(file => {
		const filePath = path.join(dir, file);
		const stat = fs.statSync(filePath);
		
		if (stat.isDirectory()) {
			findTsxFiles(filePath, fileList);
		} else if (file.endsWith('.tsx')) {
			// Convert absolute path to relative path from project root
			const relativePath = `./${path.relative('.', filePath).replace(/\\/g, '/')}`;
			fileList.push(relativePath);
		}
	});
	
	return fileList;
}

try {
	// Read the source HTML file
	let content = fs.readFileSync(srcFile, "utf8");

	if (mode === "vite") {
		// Find all .tsx files in the entire src directory
		const tsxFiles = findTsxFiles(srcDir)
			.map(filePath => `<script type="module" src="${filePath}"></script>`)
			.join("\n");

		// Replace the first line containing "VITE" with the generated script tags
		content = content.replace(/^.*VITE.*$/m, tsxFiles);

		// Remove everything between <!-- TRUNK_START --> and <!-- TRUNK_END -->
		content = content.replace(/<!-- TRUNK_START -->[\s\S]*?<!-- TRUNK_END -->/g, "");

		console.log(`✅ Generated index.html for Vite with ${tsxFiles.split('\n').length} TSX files.`);
	} else if (mode === "trunk") {
		// Remove the line containing "VITE"
		content = content.split("\n").filter(line => !line.includes("VITE")).join("\n");

		// Remove the TRUNK_START and TRUNK_END comments but keep their content
		content = content.replace(/<!-- TRUNK_START -->/g, "").replace(/<!-- TRUNK_END -->/g, "");

		console.log("✅ Generated index.html for Trunk.");
	}

	// Write the modified content to index.html
	fs.writeFileSync(outputFile, content, "utf8");
} catch (error) {
	console.error("❌ Error processing index.html:", error);
	process.exit(1);
}
