import { Link as RRLink } from "react-router";

import Container from "../components/Container";
import Link from "../components/Link";
import Logo from "../components/Logo";

const NotFound = () => (
  <Container>
    <div className="flex flex-col items-center">
      <Logo />
      <h1 className="mt-6 mb-2 text-zinc-300">Not found</h1>
      <Link asChild>
        <RRLink to="/">Go home</RRLink>
      </Link>
    </div>
  </Container>
);

export default NotFound;
