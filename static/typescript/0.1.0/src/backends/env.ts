import { env } from 'process';

export function getEnv(name: string, defaultValue?: string): string {
  const value = env[name];
  if (value === undefined) {
    if (defaultValue) {
      return defaultValue;
    } else {
      throw new Error(`Undefined env variable: ${name}`);
    }
  }

  return value;
}
