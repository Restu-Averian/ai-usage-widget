import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useEffect } from "react";
import { MockUIApp } from "./components/MockUIApp";
import { registerBackendEventInvalidation } from "./ipc/events";
import { useThemePreference } from "./lib/theme";
import "./App.css";

const queryClient = new QueryClient();

function App() {
  useThemePreference();

  useEffect(() => {
    const unregister = registerBackendEventInvalidation(queryClient);

    return () => {
      void unregister();
    };
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <MockUIApp />
    </QueryClientProvider>
  );
}

export default App;
