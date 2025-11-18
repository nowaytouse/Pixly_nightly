function mitt(n) {
  return { all: n = n || /* @__PURE__ */ new Map(), on: function(t, e) {
    var i = n.get(t);
    i && i.push(e) || n.set(t, [e]);
  }, off: function(t, e) {
    var i = n.get(t);
    i && i.splice(i.indexOf(e) >>> 0, 1);
  }, emit: function(t, e) {
    (n.get(t) || []).slice().map(function(n2) {
      n2(e);
    }), (n.get("*") || []).slice().map(function(n2) {
      n2(t, e);
    });
  } };
}
export {
  mitt as m
};
