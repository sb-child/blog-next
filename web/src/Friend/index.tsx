import { Typography } from "@mui/material";
import * as styles from "./index.module.css";

const Page = () => {
  return (
    <div className={styles.content}>
      <Typography variant="h2" gutterBottom className={styles.title}>
        友情链接
      </Typography>
    </div>
  );
};

export default Page;
