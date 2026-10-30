// The emitted contract JSON (site/src/generated) is large; importing it with
// full literal-type inference makes tsc / astro check crawl. Declare the
// generated JSON modules as `unknown` — consumers cast to the contract
// types from src/types/contract (see src/lib/data.ts).
declare module '*/generated/tierlist.json' {
  const value: unknown;
  export default value;
}
declare module '*/generated/meta.json' {
  const value: unknown;
  export default value;
}
declare module '*/generated/codes.json' {
  const value: unknown;
  export default value;
}
declare module '*/generated/history.json' {
  const value: unknown;
  export default value;
}
