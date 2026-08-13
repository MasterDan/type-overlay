import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";
import "~/index.css";
import { AppLayout } from "~/App";
import { MainView } from "~/pages/MainView";
import { SettingsPage } from "~/pages/SettingsPage";

const root = document.getElementById("root");
if (!root) throw new Error("root element not found");

render(
  () => (
    <Router root={AppLayout}>
      <Route path="/" component={MainView} />
      <Route path="/settings" component={SettingsPage} />
    </Router>
  ),
  root,
);
