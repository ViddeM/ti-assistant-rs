(() => {
  const originalFetch = window.fetch;
  if (!originalFetch) return;
  const root = document.documentElement;
  const mb = (bytes) => Math.round(bytes / 1e6);

  const track = (res) => {
    const encoding = res.headers.get("content-encoding");
    const length = Number(res.headers.get("content-length"));
    const total = (!encoding || encoding === "identity") && length > 0 ? length : 0;
    if (total) root.dataset.wasmProgress = "known";

    let loaded = 0;
    let frame = 0;
    const draw = () => {
      frame = 0;
      if (total) root.style.setProperty("--wasm-progress", Math.min(loaded / total, 1));
      const label = document.querySelector(".app-loading-label");
      if (label) label.textContent = total ? `${mb(loaded)} / ${mb(total)} MB` : `${mb(loaded)} MB`;
    };

    const reader = res.clone().body.getReader();
    const pump = () =>
      reader.read().then(({ done, value }) => {
        if (done) {
          if (frame) cancelAnimationFrame(frame);
          draw();
          root.dataset.wasmProgress = "done";
          return;
        }
        loaded += value.byteLength;
        if (!frame) frame = requestAnimationFrame(draw);
        return pump();
      });
    return pump();
  };

  window.fetch = function (input, init) {
    const result = originalFetch.call(this, input, init);
    const url = input instanceof Request ? input.url : String(input);
    if (url.endsWith(".wasm")) {
      window.fetch = originalFetch;
      result
        .then((res) => {
          if (res.ok && res.body) return track(res);
        })
        .catch(() => {});
    }
    return result;
  };
})();
