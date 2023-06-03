import Header from "@/components/header"
import {useSelector} from "react-redux";
import {selectAuthState} from "@/redux/auth";
import {isSome} from "fp-ts/Option";
import {extractAuthUsername} from "@/domain/auth";

export default function Layout({ children }) {
  const authState = useSelector(selectAuthState);
  const isActive = isSome(authState.user)
  const username = extractAuthUsername(authState)

  return (
    <div>
      <Header isActive={isActive} username={username} />
      <main>{children}</main>
    </div>
  );
}