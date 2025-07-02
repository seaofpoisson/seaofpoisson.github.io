const width = 100;
const height = 100;
const color = d3.scaleOrdinal(d3.schemeCategory20);
const node_radius = 5;

function render_graph_cocycle(container, cocycle)
{
    for (let i = 0; i < cocycle.terms.length; i++) {
        let [coeff, graph] = cocycle.terms[i];
        render_graph_cocycle_term(container, coeff, graph, cocycle.num_vertices);
    }
}

function render_graph_cocycle_term(container, coeff, graph, num_vertices)
{
    // Create graph for d3.
    let d3graph = {
        nodes: [],
        links: []
    };
    for (let j = 0; j < graph.length; j++) {
        let [v, w] = graph[j];
        d3graph.links.push({source: v, target: w});
    }
    for (let v = 0; v < num_vertices; v++) {
        d3graph.nodes.push({name: v.toString(), group: 1});
    }
    // Add dummy nodes for the corners of the bounds.
    const real_nodes = d3graph.nodes.slice(0);
    const top_left = {x: 0, y: 0, fixed: true, fixedWeight: 100};
    const top_left_idx = d3graph.nodes.push(top_left) - 1;
    const bottom_right = {x: width, y: height, fixed: true, fixedWeight: 100};
    const bottom_right_idx = d3graph.nodes.push(bottom_right) - 1;
    // Add constraints to stay inside the bounds.
    let constraints = [];
    for (let i = 0; i < real_nodes.length; i++) {
        constraints.push({ axis: 'x', type: 'separation', left: top_left_idx, right: i, gap: node_radius });
        constraints.push({ axis: 'y', type: 'separation', left: top_left_idx, right: i, gap: node_radius });
        constraints.push({ axis: 'x', type: 'separation', left: i, right: bottom_right_idx, gap: node_radius });
        constraints.push({ axis: 'y', type: 'separation', left: i, right: bottom_right_idx, gap: node_radius });
    }
    // Use cola for the graph layout.
    const d3cola = cola.d3adaptor(d3).size([width, height]);
    d3cola.nodes(d3graph.nodes)
        .links(d3graph.links)
        .constraints(constraints)
        .avoidOverlaps(true)
        .jaccardLinkLengths(30,0.7)
        .start(30);
    // Render the graph to an SVG using d3.
    const svg = d3.create("svg")
        .attr("width", width)
        .attr("height", height)
        .style("vertical-align", "middle");
    let link = svg.selectAll(".link")
        .data(d3graph.links)
        .enter().append("line")
        .attr("class", "link")
        .style("stroke", "white")
        .style("stroke-width", "2");
    let node = svg.selectAll(".node")
        .data(real_nodes)
        .enter().append("circle")
        .attr("class", "node")
        .attr("r", node_radius)
        .style("fill", "white")
        .call(d3cola.drag);
    node.append("title")
        .text(function (d) { return d.name; });
    d3cola.on("tick", function () {
        link.attr("x1", function (d) { return d.source.x; })
            .attr("y1", function (d) { return d.source.y; })
            .attr("x2", function (d) { return d.target.x; })
            .attr("y2", function (d) { return d.target.y; });
        node.attr("cx", function (d) { return d.x; })
            .attr("cy", function (d) { return d.y; });
    });
    // Put the graph and its coefficient in a div.
    let div = document.createElement("div");
    div.style.lineHeight = height + "px";
    const has_minus = coeff[0] === "-";
    let coeff_latex = "";
    if (coeff.indexOf("/") === -1) {
        coeff_latex = (has_minus ? coeff.substring(1) : coeff);
    } else {
        const [num, denom] = coeff.split("/");
        coeff_latex = "\\frac{" + (has_minus ? num.substring(1) : num) + "}{" + denom + "}";
    }
    coeff_latex = (has_minus ? "-" : "+") + coeff_latex;
    katex.render(coeff_latex, div, {
        throwOnError: false
    });
    div.appendChild(svg.node());
    container.appendChild(div);
}
