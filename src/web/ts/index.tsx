import { render } from "solid-js/web";
import "../css/canvas.css";
// pinit.run();
console.log("Heelo from TypeScript!")

function App() {
	return (
		<div>
			<h1>Hello, World! </h1>
		</div>
	);
}

render(() => <App />, document.getElementById("root") as HTMLElement);

