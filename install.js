#!/usr/bin/env node

const { existsSync, chmodSync, mkdirSync } = require("fs");
const https = require("https");
const path = require("path");
const os = require("os");

const version = require("./package.json").version;

function getPlatform() {
	const type = os.type();
	const arch = os.arch();

	if (type === "Windows_NT") {
		return arch === "x64" ? "x86_64-pc-windows-msvc" : "i686-pc-windows-msvc";
	}

	if (type === "Linux") {
		return arch === "x64"
			? "x86_64-unknown-linux-gnu"
			: arch === "arm64"
				? "aarch64-unknown-linux-gnu"
				: "i686-unknown-linux-gnu";
	}

	if (type === "Darwin") {
		return arch === "arm64" ? "aarch64-apple-darwin" : "x86_64-apple-darwin";
	}

	throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getDownloadUrl(platform) {
	const ext = platform.includes("windows") ? ".exe" : "";
	return `https://github.com/nehu3n/tazk/releases/download/v${version}/tazk-${platform}${ext}`;
}

async function downloadBinary() {
	const platform = getPlatform();
	const url = getDownloadUrl(platform);
	const binDir = path.join(__dirname, "bin");
	const binPath = path.join(
		binDir,
		platform.includes("windows") ? "tazk.exe" : "tazk",
	);

	console.log("🐕 Installing Tazk...");
	console.log(`📥 Downloading from: ${url}`);

	if (!existsSync(binDir)) {
		mkdirSync(binDir, { recursive: true });
	}

	return new Promise((resolve, reject) => {
		const file = require("fs").createWriteStream(binPath);

		https
			.get(url, (response) => {
				if (response.statusCode === 200) {
					response.pipe(file);
					file.on("finish", () => {
						file.close();
						try {
							chmodSync(binPath, "755");
							console.log("✅ Tazk installed successfully!");
							console.log(`📍 Binary location: ${binPath}`);
							console.log("🚀 Try running: tazk --help");
							resolve();
						} catch (error) {
							reject(error);
						}
					});
				} else if (response.statusCode === 302 || response.statusCode === 301) {
					https
						.get(response.headers.location, (redirectResponse) => {
							redirectResponse.pipe(file);
							file.on("finish", () => {
								file.close();
								chmodSync(binPath, "755");
								console.log("✅ Tazk installed successfully!");
								resolve();
							});
						})
						.on("error", reject);
				} else {
					reject(
						new Error(
							`Failed to download: ${response.statusCode} ${response.statusMessage}`,
						),
					);
				}
			})
			.on("error", reject);

		file.on("error", reject);
	});
}

async function main() {
	try {
		await downloadBinary();
	} catch (error) {
		console.error("❌ Installation failed:", error.message);
		console.error(
			"💡 You can try installing via Cargo instead: cargo install tazk",
		);
		process.exit(1);
	}
}

if (require.main === module) {
	main();
}
