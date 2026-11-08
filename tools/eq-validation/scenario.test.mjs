import test from 'node:test';
import assert from 'node:assert/strict';
import { parseScenario } from './scenario.mjs';

test('taps include an ordered release and can contain an intervening screenshot', () => {
  assert.deepEqual(parseScenario([{at: 1, tap: [54, 110]}, {at: 1.05, shot: 'pressed'}], 5), [
    {at: 1, pointer: [1, 54, 110]}, {at: 1.05, shot: 'pressed'}, {at: 1.12, pointer: [2, 54, 110]},
  ]);
});

test('invalid scenarios cannot enter keys or write screenshots outside the output directory', () => {
  for (const value of [null, [{at: 0, key: 65}], [{at: 0, shot: '../secret'}],
    [{at: 0, tap: [-1, 1]}], [{at: 0, tap: [0.5, 1]}],
    [{at: 1, shot: 'later'}, {at: 0, shot: 'earlier'}], [{at: 4.95, tap: [1, 1]}],
    [{at: 0, shot: 'both', tap: [1, 1]}]]) {
    assert.throws(() => parseScenario(value, 5));
  }
});
