import {
  Badge,
  Breadcrumbs,
  Button,
  IconButton,
  InputAdornment,
  Stack,
  TextField,
  Tooltip,
} from "@mui/material";
import * as styles from "./index.module.css";
import NotificationsIcon from "@mui/icons-material/Notifications";
import SearchIcon from "@mui/icons-material/Search";
import FavoriteIcon from "@mui/icons-material/Favorite";
import InfoIcon from "@mui/icons-material/Info";
import NavigateNextIcon from "@mui/icons-material/NavigateNext";
import { useSignal, useComputed } from "@preact/signals";

const SearchBox = () => {
  const searchText = useSignal("");
  const searchTextEmpty = useComputed(() => searchText.value.length <= 0);
  const searchFocus = useSignal(false);
  const searchShowLabel = useComputed(
    () => searchTextEmpty.value && !searchFocus.value,
  );
  const searchLabelText = "色妹妹的船新博客";
  const searchLabelText2 = "键入以搜索";
  const searchFocused = () => {
    searchFocus.value = true;
  };
  const searchBlured = () => {
    searchFocus.value = false;
  };

  return (
    <TextField
      // hiddenLabel={!searchShowLabel.value}
      onFocus={searchFocused}
      onBlur={searchBlured}
      defaultValue={""}
      value={searchText}
      onChange={(ev) => {
        // @ts-ignore
        searchText.value = ev.target?.value;
      }}
      label={searchShowLabel.value ? searchLabelText : searchLabelText2}
      variant="filled"
      size="small"
      slotProps={{
        input: {
          startAdornment: searchShowLabel.value ? null : (
            <InputAdornment position="start">
              <SearchIcon />
            </InputAdornment>
          ),
        },
      }}
    />
  );
};

const App = () => {
  return (
    <div className={styles.container}>
      <div className={styles.placeholder} />
      <div className={styles.body}>
        <div className={styles.body_logo}>
          <Tooltip
            title="回到首页"
            slotProps={{
              popper: {
                modifiers: [
                  {
                    name: "offset",
                    options: {
                      offset: [0, -6],
                    },
                  },
                ],
              },
            }}
          >
            <div className={styles.body_logo_inner}>logo</div>
          </Tooltip>
        </div>
        <div className={styles.body_search}>
          <SearchBox />
        </div>
        <div className={styles.body_title}>
          <Breadcrumbs
            separator={<NavigateNextIcon fontSize="small" />}
            aria-label="breadcrumb"
          >
            <div>首页</div>
            <div>友情链接</div>
          </Breadcrumbs>
        </div>
        <div className={styles.body_links}>
          <Stack direction="row" spacing={1}>
            <Tooltip title="友情链接">
              <IconButton color="secondary">
                <FavoriteIcon />
              </IconButton>
            </Tooltip>
            <Tooltip title="关于博主">
              <IconButton color="primary">
                <InfoIcon />
              </IconButton>
            </Tooltip>
          </Stack>
        </div>
        <div className={styles.body_right}>
          <Tooltip title="查看通知">
            <IconButton>
              <Badge badgeContent={2} color="error">
                <NotificationsIcon />
              </Badge>
            </IconButton>
          </Tooltip>
        </div>
      </div>
    </div>
  );
};

export default App;
