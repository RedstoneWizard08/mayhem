import { ExecutorContext } from '@nrwl/devkit';
import { buildCommand } from '../../utils/build-command';
import { cargoRunCommand } from '../../utils/cargo';
import { RunExecutorSchema } from './schema';

export default async function* runExecutor(
  options: RunExecutorSchema,
  context: ExecutorContext
) {
  const command = buildCommand('run', options, context);

  const { success } = await cargoRunCommand(options.cross, ...command);
  yield {
    success,
  };
}
