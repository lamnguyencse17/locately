import React from "react";
import { NativeBaseProvider, Box } from "native-base";

export default function App() {
  return (
    <NativeBaseProvider>
      <Box safeArea>Hello world</Box>
    </NativeBaseProvider>
  );
}
