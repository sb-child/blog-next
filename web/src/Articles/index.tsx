import { Typography } from "@mui/material";
import * as styles from "./index.module.css";

const Page = () => {
  return (
    <div className={styles.content}>
      <Typography variant="h2" gutterBottom className={styles.title}>
        文章列表
      </Typography>
    </div>
  );
};

export default Page;
