import type { ComponentChildren } from "preact";
import * as styles from "./index.module.css";
import { animate } from "animejs";
import TopBar from "../components/TopBar";

const App = (props: { children?: ComponentChildren }) => {
  return (
    <div className={styles.layout}>
      <TopBar />
      <div className={styles.content}>{props.children}</div>
    </div>
  );
};

export default App;
