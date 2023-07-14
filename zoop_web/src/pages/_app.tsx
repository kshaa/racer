import '@/styles/globals.css'
import type { AppProps } from 'next/app'
import {wrapper} from "@/redux/store";
import {createTheme, ThemeProvider} from "@mui/material/styles";
import Layout from "@/components/layout";

const theme = createTheme({
  palette: {
    primary: {
      main: '#ffbc4a',
    },
    secondary: {
      main: '#ffdca4'
    }
  }
});

export function App({ Component, pageProps }: AppProps) {
  return <ThemeProvider theme={theme}>
    <Layout>
      <Component {...pageProps} />
    </Layout>
  </ThemeProvider>
}

export default wrapper.withRedux(App)