import { Typography } from "@mui/material";
import * as styles from "./index.module.css";
import { Graph } from "@antv/g6";
import { useEffect, useRef } from "preact/hooks";

const Page = () => {
  const graphRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const graph = new Graph({
      // biome-ignore lint/style/noNonNullAssertion: <explanation>
      container: graphRef.current!,
      // width: 500,
      // TODO: resize event
      height: 500,
      data: {
        nodes: [
          {
            id: "node-1",
            style: { x: 50, y: 100 },
          },
          {
            id: "node-2",
            style: { x: 150, y: 100 },
          },
        ],
        edges: [{ id: "edge-1", source: "node-1", target: "node-2" }],
      },
      behaviors: ["drag-canvas", "zoom-canvas", "drag-element"],
      animation: {
        duration: 500,
      },
    });
    graph.render();
    graph.resize();
    return () => {
      graph.destroy();
    };
  }, []);

  return (
    <div className={styles.content}>
      <Typography variant="h2" gutterBottom className={styles.title}>
        友情链接
      </Typography>
      {/* TODO: buttons on the right */}
      <div ref={graphRef} className={styles.graph}>
        aa
      </div>
    </div>
  );
};

export default Page;
