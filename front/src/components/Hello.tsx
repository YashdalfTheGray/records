import { h } from 'preact';

interface HelloProps {
  name: string;
}

export const Hello = ({ name }: HelloProps) => {
  return <p>Hello, {name}!</p>;
};
