import { Routes, Route } from 'react-router-dom';
import { MainLayout } from './components/layout/main-layout';
import { HomePage } from './components/pages/home-page';
import { ResultPage } from './components/pages/result-page';
import { DocsPage } from './components/pages/docs-page';
import { SettingsPage } from './components/pages/settings-page';

function App() {
  return (
    <MainLayout>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/result/:jobId" element={<ResultPage />} />
        <Route path="/docs" element={<DocsPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Routes>
    </MainLayout>
  );
}

export default App;
