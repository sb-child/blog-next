import { render } from "preact";
import App from "./App";
import {
	LocationProvider,
	ErrorBoundary,
	Route,
	Router,
	lazy,
} from "preact-iso";
import NotFound from "./NotFound";
import Layout from "./Layout";
import Background from "./Background";
import "./index.css";

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
		<div className="site-root">
			<Background />
			<Layout>
				<SiteRouter />
			</Layout>
		</div>
	);
};

const root = document.getElementById("root");
if (root) {
	render(<SiteRoot />, root);
}
