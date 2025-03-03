import { render } from "solid-js/web";

console.log("Hello from TypeScript!")

function App() {
	return (
		<div>
			<h1>Hello, World! </h1>
		</div>
	);
}

render(() => <App />, document.getElementById("root") as HTMLElement);

