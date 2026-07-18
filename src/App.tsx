import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MockUIApp } from "./components/MockUIApp";
import { useThemePreference } from "./lib/theme";
import "./App.css";

const queryClient = new QueryClient();

function App() {
  useThemePreference();

  return (
    <QueryClientProvider client={queryClient}>
      <MockUIApp />
    </QueryClientProvider>
  );
}

export default App;
