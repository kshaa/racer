import {Typography} from "@mui/material";

export interface HeaderProps {
  isActive: boolean,
  username: string
}

export default function Header(props: HeaderProps) {
  return (
    <div id="navigation" className={`${props.isActive ? "active" : ""}`}>
      <div className="container">
        <Typography className="logo">Town Racer</Typography>
        <Typography className="username">Logged in as {props.username}</Typography>
      </div>
    </div>
  )
}