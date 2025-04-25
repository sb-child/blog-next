import { Typography } from "@mui/material";
import * as styles from "./index.module.css";
import { useRoute } from "preact-iso";

const Page = () => {
  const route = useRoute();
  return (
    <div className={styles.content}>
      <Typography variant="h2" gutterBottom className={styles.title}>
        文章标题喵啊啊~
      </Typography>
      <div>{route.path}</div>
      <div>{route.params.id}</div>
    </div>
  );
};

export default Page;
