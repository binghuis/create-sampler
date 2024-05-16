#! /usr/bin/env node

import * as p from '@clack/prompts';
import kleur from 'kleur';

const cancel = (message?: string) => {
  process.exit(0);
};

async function main() {}

main().catch((err) => {
  p.log.error(kleur.red(err.message));
  process.exit(1);
});
