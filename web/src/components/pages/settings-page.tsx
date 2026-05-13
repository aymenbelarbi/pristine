import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Separator } from '@/components/ui/separator';
import { useUIStore } from '@/stores/uiStore';
import { BudgetSlider } from '@/components/forms/budget-slider';
import { PatternInput } from '@/components/forms/pattern-input';
import { Save, RotateCcw } from 'lucide-react';

export function SettingsPage() {
  const { theme, setTheme } = useUIStore();
  const [apiUrl, setApiUrl] = useState(localStorage.getItem('apiUrl') || '/api/v1');
  const [defaultBudget, setDefaultBudget] = useState(50000);
  const [defaultPackType, setDefaultPackType] = useState('overview');
  const [includePatterns, setIncludePatterns] = useState<string[]>([]);
  const [excludePatterns, setExcludePatterns] = useState<string[]>(['node_modules', '.git', 'dist', 'build']);
  const [autoDownload, setAutoDownload] = useState(false);
  const [showLineNumbers, setShowLineNumbers] = useState(true);

  const handleSave = () => {
    localStorage.setItem('apiUrl', apiUrl);
    localStorage.setItem('defaultBudget', defaultBudget.toString());
    localStorage.setItem('defaultPackType', defaultPackType);
    localStorage.setItem('autoDownload', autoDownload.toString());
    localStorage.setItem('showLineNumbers', showLineNumbers.toString());
  };

  const handleReset = () => {
    setApiUrl('/api/v1');
    setDefaultBudget(50000);
    setDefaultPackType('overview');
    setIncludePatterns([]);
    setExcludePatterns(['node_modules', '.git', 'dist', 'build']);
    setAutoDownload(false);
    setShowLineNumbers(true);
  };

  return (
    <div className="max-w-2xl mx-auto space-y-8">
      <div>
        <h1 className="text-3xl font-bold">Settings</h1>
        <p className="text-muted-foreground mt-2">
          Configure your Pristine experience.
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
          <CardDescription>Customize the look and feel</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Theme</Label>
              <p className="text-sm text-muted-foreground">
                Select your preferred color scheme
              </p>
            </div>
            <Select value={theme} onValueChange={setTheme}>
              <SelectTrigger className="w-[180px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="light">Light</SelectItem>
                <SelectItem value="dark">Dark</SelectItem>
                <SelectItem value="system">System</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>API Configuration</CardTitle>
          <CardDescription>Configure the API endpoint</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="api-url">API URL</Label>
            <Input
              id="api-url"
              value={apiUrl}
              onChange={(e) => setApiUrl(e.target.value)}
              placeholder="/api/v1"
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Default Options</CardTitle>
          <CardDescription>Set default values for new ingestions</CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-2">
            <Label>Default Pack Type</Label>
            <Select value={defaultPackType} onValueChange={setDefaultPackType}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="overview">Overview Pack</SelectItem>
                <SelectItem value="task">Task Pack</SelectItem>
                <SelectItem value="review">Review Pack</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <BudgetSlider
            label="Default Token Budget"
            value={defaultBudget}
            onChange={setDefaultBudget}
          />

          <PatternInput
            label="Default Include Patterns"
            patterns={includePatterns}
            onChange={setIncludePatterns}
          />

          <PatternInput
            label="Default Exclude Patterns"
            patterns={excludePatterns}
            onChange={setExcludePatterns}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Display Options</CardTitle>
          <CardDescription>Customize how results are displayed</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Auto-download Results</Label>
              <p className="text-sm text-muted-foreground">
                Automatically download results when processing completes
              </p>
            </div>
            <Switch checked={autoDownload} onCheckedChange={setAutoDownload} />
          </div>
          <Separator />
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label>Show Line Numbers</Label>
              <p className="text-sm text-muted-foreground">
                Display line numbers in code blocks
              </p>
            </div>
            <Switch checked={showLineNumbers} onCheckedChange={setShowLineNumbers} />
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end gap-4">
        <Button variant="outline" onClick={handleReset}>
          <RotateCcw className="h-4 w-4 mr-2" />
          Reset to Defaults
        </Button>
        <Button onClick={handleSave}>
          <Save className="h-4 w-4 mr-2" />
          Save Changes
        </Button>
      </div>
    </div>
  );
}
