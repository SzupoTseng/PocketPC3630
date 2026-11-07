// Timed pointer-only scenarios. They cannot enter or retain registration codes.
export function parseScenario(value, seconds) {
  if (!Array.isArray(value) || value.length > 100) throw new Error('Scenario must contain at most 100 actions');
  let previous = -1;
  return value.flatMap((action) => {
    if (!action || typeof action !== 'object' || Array.isArray(action)) throw new Error('Invalid action');
    const { at, tap, shot } = action;
    if (Object.keys(action).some((key) => !['at', 'tap', 'shot'].includes(key))) throw new Error('Unknown action field');
    if (!Number.isFinite(at) || at < 0 || at < previous || at >= seconds) throw new Error('Action times must be ordered and inside the run');
    previous = at;
    if ((tap !== undefined) === (shot !== undefined)) throw new Error('Choose exactly one tap or shot');
    if (shot !== undefined) {
      if (typeof shot !== 'string' || !/^[a-z0-9_-]{1,64}$/i.test(shot)) throw new Error('Invalid screenshot name');
      return [{ at, shot }];
    }
    if (!Array.isArray(tap) || tap.length !== 2 || tap.some((n) => !Number.isInteger(n) || n < 0 || n > 65535)) {
      throw new Error('Invalid tap coordinates');
    }
    if (at + 0.12 >= seconds) throw new Error('Tap release must be inside the run');
    return [{ at, pointer: [1, ...tap] }, { at: at + 0.12, pointer: [2, ...tap] }];
  }).sort((a, b) => a.at - b.at);
}
