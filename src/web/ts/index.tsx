import { render } from "solid-js/web";
import "../css/plinth.css";
// pinit.run();
console.log("Heelo from TypeScript!")

function App() {
	return (
		<div class="HelloWorld">
			<h1>Hello, World! </h1>
		</div>
	);
}

render(() => <App />, document.getElementById("root") as HTMLElement);

