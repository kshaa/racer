import Header from "@/components/header"
import {useSelector} from "react-redux";
import {selectAuthState} from "@/redux/auth";
import {isSome} from "fp-ts/Option";
import {extractAuthUsername} from "@/domain/auth";

export interface LayoutProps {
  children: any
}
export default function Layout(props: LayoutProps) {
  const authState = useSelector(selectAuthState);
  const isActive = isSome(authState.user)
  const username = extractAuthUsername(authState)

  return (
    <div>
      <Header isActive={isActive} username={username} />
      <main>{props.children}</main>
    </div>
  );
}