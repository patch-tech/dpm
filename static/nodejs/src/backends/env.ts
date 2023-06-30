import { env } from 'process';

/**
 * Returns the value of the environment variable `name`, if found. Else, returns
 * `defaultValue` if provided, undefined otherwise.
 * @param name Name of environment variable.
 * @param defaultValue
 * @returns Value of environment variable, else specified defaultValue or undefined.
 */
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
