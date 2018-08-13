var rpc = {
    invoke: function(arg) { window.external.invoke(JSON.stringify(arg)); },
    init: function() { rpc.invoke({cmd: 'init'}) },
    render: function(state) {
        document.getElementById("content").innerHTML = "<p>" + state + "</p>";
    }
};

var runAlert = function(target) {
    if (target.className === "value") {
        window.alert("Value with id: " + target.id);
    } else if (target.className === "expression") {
        window.alert("Expression clicked");
};

window.onload = function() {
    rpc.init();
    
    document.getElementById("content").addEventListener('click', function(event) {
        runAlert(event.target);
    });
}
