#!/usr/bin/env node
/**
 * Hello World example for Nova
 * A simple demonstration of JavaScript code in the Nova project structure.
 */

function greet() {
    return "Hello from Nova! 🌟";
}

function main() {
    const message = greet();
    console.log(message);
    console.log("Welcome to your new beginning with Nova!");
}

// Run the example
if (require.main === module) {
    main();
}

module.exports = { greet, main };