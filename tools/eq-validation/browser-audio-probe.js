// Test-only preload injected by CDP. Observe the real Web Audio output callback.
// Keep at most ten seconds of PCM locally; never put game audio in source.
(() => {
  const Context = window.AudioContext || window.webkitAudioContext;
  if (!Context) return;
  const create = Context.prototype.createScriptProcessor;
  const chunks = [];
  let captured = 0, rate = 0, buffers = 0, samples = 0, nonzero = 0, peak = 0, square = 0;
  Context.prototype.createScriptProcessor = function (...args) {
    const node = create.apply(this, args), connect = node.connect;
    rate = this.sampleRate;
    node.connect = function (...targets) {
      const process = node.onaudioprocess;
      node.onaudioprocess = function (event) {
        process.call(this, event);
        const l = event.outputBuffer.getChannelData(0), r = event.outputBuffer.getChannelData(1);
        const pcm = new Int16Array(l.length * 2);
        let audible = false;
        for (let i = 0; i < l.length; i++) for (let c = 0; c < 2; c++) {
          const value = c ? r[i] : l[i];
          samples++; if (Math.abs(value) > 1e-5) { nonzero++; audible = true; }
          peak = Math.max(peak, Math.abs(value)); square += value * value;
          pcm[i * 2 + c] = Math.round(Math.max(-1, Math.min(1, value)) * 32767);
        }
        buffers++;
        // Preserve audible excerpts instead of ending with ten seconds of
        // silence after EverQuest stops its title music on entering the world.
        if (audible && captured + pcm.length <= rate * 20) {
          chunks.push(pcm); captured += pcm.length;
        }
      };
      return connect.apply(this, targets);
    };
    return node;
  };
  window.__eqAudioReport = () => {
    const bytes = new Uint8Array(44 + captured * 2), view = new DataView(bytes.buffer);
    const text = (at, value) => [...value].forEach((c, i) => bytes[at + i] = c.charCodeAt(0));
    text(0, 'RIFF'); view.setUint32(4, bytes.length - 8, true); text(8, 'WAVEfmt ');
    view.setUint32(16, 16, true); view.setUint16(20, 1, true); view.setUint16(22, 2, true);
    view.setUint32(24, rate, true); view.setUint32(28, rate * 4, true);
    view.setUint16(32, 4, true); view.setUint16(34, 16, true); text(36, 'data');
    view.setUint32(40, captured * 2, true);
    let at = 44; for (const chunk of chunks) for (const value of chunk) { view.setInt16(at, value, true); at += 2; }
    let binary = ''; for (let i = 0; i < bytes.length; i += 16384) binary += String.fromCharCode(...bytes.subarray(i, i + 16384));
    return {scope:'Real Web Audio output callback; WAV contains up to ten seconds of audible excerpts, not a continuous listening test',capturedSamples:captured,rate,channels:2,buffers,samples,nonzero,peak,rms:Math.sqrt(square/Math.max(1,samples)),wav:btoa(binary)};
  };
})();
