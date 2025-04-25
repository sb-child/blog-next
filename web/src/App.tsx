import { Alert, Chip, Divider, Typography } from "@mui/material";
import * as styles from "./app.module.css";

const App = () => {
  return (
    <div className={styles.content}>
      <div className={styles.alert_container}>
        <Alert severity="warning">
          本网站暂时仅适配桌面端屏幕比例，为您带来不便敬请谅解喵
        </Alert>
      </div>
      <Typography variant="h1" gutterBottom className={styles.title}>
        嗨，别来无恙喵!
      </Typography>
      <Divider textAlign="left">
        <Chip label="最新文章" size="medium" />
      </Divider>
      <div className={styles.articles_container}>
        <div>aaa</div>
      </div>
      <Divider textAlign="left">
        <Chip label="喵喵喵" size="medium" />
      </Divider>
      <div className={styles.articles_container}>
        <div>aaa</div>
      </div>
    </div>
  );
};

export default App;
