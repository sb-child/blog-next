import { render } from "preact";
import {
  ErrorBoundary,
  LocationProvider,
  Route,
  Router,
  lazy,
} from "preact-iso";
import App from "./App";
import Background from "./Background";
import Layout from "./Layout";
import NotFound from "./NotFound";
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
const LazyNotFound = lazy(async () => NotFound);

const SiteRouter = () => {
  return (
    <div className="router">
      <LocationProvider>
        <ErrorBoundary>
          <Router>
            <Route path="/" component={LazyIndex} />

            {/* <Route path="/profiles" component={Profiles} />
						<Route path="/profile/:id" component={Profile} /> */}

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
