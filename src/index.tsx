/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

import MainScreen from "./page/MainScreen";
import PlayPanel from "./page/PlayPanel";
import "./styles.css";
import Settings from "./page/Settings";

const rootElement = document.getElementById("root");

if (rootElement) {
  render(
    () => (
      <Router>
        <Route path="/" component={MainScreen} />
        <Route path="/play-panel" component={PlayPanel} />
        <Route path="/settings" component={Settings} />
      </Router>
    ),
    rootElement,
  );
}
