import logo from "./logo.svg";
import "./App.css";
import { useWasm } from "./useWasm";

function App() {
  const { loaded, instance, error } = useWasm("my-wasm.wasm");
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
      <div className="App">
        {loaded && instance.exports.addString("hello", "wasm")}
        {error && error.message}
      </div>
    </div>
  );
}

export default App;
