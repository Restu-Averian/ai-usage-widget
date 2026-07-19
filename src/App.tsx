import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useEffect } from "react";
import { MockUIApp } from "./components/MockUIApp";
import { registerBackendEventInvalidation } from "./ipc/events";
import "./App.css";

const queryClient = new QueryClient();

function App() {
  useEffect(() => {
    document.documentElement.dataset.theme = "dark";
    document.documentElement.style.colorScheme = "dark";
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
