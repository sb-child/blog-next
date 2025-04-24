import type { ComponentChildren } from "preact";
import "./index.css";
import { animate } from 'animejs';

const App = (props: { children?: ComponentChildren }) => {
  return (
    <div className="layout">
      <h1>sbchild blog</h1>
      {props.children}
    </div>
  );
};

export default App;
