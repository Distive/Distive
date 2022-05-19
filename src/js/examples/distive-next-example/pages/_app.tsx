import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { ChakraProvider } from '@chakra-ui/react'
import { Connect2ICProvider } from "@connect2ic/react"
import "@connect2ic/core/style.css"

function MyApp({ Component, pageProps }: AppProps) {
  return  <ChakraProvider>
      <Component {...pageProps} />
    </ChakraProvider>

}

export default MyApp
