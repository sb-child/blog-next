import { render } from "preact";
import {
  ErrorBoundary,
  LocationProvider,
  Route,
  Router,
  lazy,
} from "preact-iso";
import App from "./App";
import Article from "./Article";
import Articles from "./Articles";
import Background from "./Background";
import Layout from "./Layout";
import NotFound from "./NotFound";
import Friends from "./Friends";
import Friend from "./Friend";
import "./index.css";

import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

const LazyIndex = lazy(async () => App);
const LazyArticle = lazy(async () => Article);
const LazyArticles = lazy(async () => Articles);
const LazyFriends = lazy(async () => Friends);
const LazyFriend = lazy(async () => Friend);
const LazyNotFound = lazy(async () => NotFound);

const SiteRouter = () => {
  return (
    <div className="router">
      <LocationProvider>
        <ErrorBoundary>
          <Router>
            <Route path="/" component={LazyIndex} />
            <Route path="/article/:id" component={LazyArticle} />
            <Route path="/articles" component={LazyArticles} />
            <Route path="/friend/:id" component={LazyFriend} />
            <Route path="/friends" component={LazyFriends} />

            <Route component={LazyNotFound} default />
          </Router>
        </ErrorBoundary>
      </LocationProvider>
    </div>
  );
};

const SiteRoot = () => {
  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <div className="site-root">
        <Background />
        <Layout>
          <SiteRouter />
        </Layout>
      </div>
    </ThemeProvider>
  );
};

const root = document.getElementById("root");
if (root) {
  render(<SiteRoot />, root);
}
