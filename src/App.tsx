import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Todo {
  id: string
  description: string
}

function App() {
  const [data, setData] = useState<Todo[] | undefined>();
  const [error, setError] = useState<string | undefined>();

  function connect() {
    invoke("query_todos").then((msg: any) => {
      console.log('THEN', msg);
      setData(msg as Todo[])
    }).catch(err => setError(`ERR: ${err}`));
  }

  return (
    <div className="container">
      <h1>Welcome to Rust TODO App!</h1>
      <div className="row">
        <div>
          <button type="button" onClick={() => connect()}>
            Greet
          </button>
        </div>
      </div>
      {data && data.map(todo => (
        <section style={{ display: 'block' }}>
          <span><i>{todo.id}</i></span>&nbsp; - &nbsp;
          <span>{todo.description}</span>
        </section>
      ))}
      {error && (<p style={{ color: 'red' }}>{error}</p>)}
    </div>
  );
}

export default App;
