import { AlertCircle, X } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';

interface ErrorAlertProps {
  title?: string;
  message: string;
  onDismiss?: () => void;
  variant?: 'default' | 'destructive';
}

export function ErrorAlert({
  title = 'Error',
  message,
  onDismiss,
  variant = 'destructive',
}: ErrorAlertProps) {
  return (
    <Alert variant={variant}>
      <AlertCircle className="h-4 w-4" />
      <AlertTitle>{title}</AlertTitle>
      <AlertDescription className="flex items-center justify-between">
        <span>{message}</span>
        {onDismiss && (
          <Button
            variant="ghost"
            size="sm"
            className="h-6 w-6 p-0"
            onClick={onDismiss}
          >
            <X className="h-4 w-4" />
          </Button>
        )}
      </AlertDescription>
    </Alert>
  );
}
