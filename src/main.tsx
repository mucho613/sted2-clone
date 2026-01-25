import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router";
import "./input.css";
import MainScreen from "./pages/MainScreen";
import TrackEditScreen from "./pages/TrackEditScreen";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route index element={<MainScreen />} />
        <Route path="/track-edit" element={<TrackEditScreen />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);
