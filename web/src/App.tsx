import {
  createBrowserRouter,
  createRoutesFromElements,
  Outlet,
  Route,
  RouterProvider,
} from "react-router";
import Home from "./pages/Home";
import Room from "./pages/Room";
import NotFound from "./pages/NotFound";

const RoutesJSX = (
  <Route path="/" element={<Outlet />}>
    <Route path="/" element={<Home />} />
    <Route path="/room/:id" element={<Room />} />
    <Route path="*" element={<NotFound />} />
  </Route>
);

const routes = createRoutesFromElements(RoutesJSX);
const router = createBrowserRouter(routes);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
