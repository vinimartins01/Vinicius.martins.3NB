use petgraph::dot::{Dot, Config}
use petgraph::graph::Graph;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()>{
    let mut graph = Graph::<&str, &str, &str>::new()>

    //adiciona Nós
    let a = graph.add_node("a");
    let b = graph.add_node("B");
    let c = graph.add_node("c");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");

        //Arestas
        graph.add_edge(a, b, "");
        graph.add_edge(b, c, "");
        graph.add_edge(c, d, "");
        graph.add_edge(d, e, "");
        graph.add_edge(e, f, "");

        // .dot
        let dot_output = format!("{:?}", Dot::with_config(&graph, &config::EdgeNoLabel))

        let mut file = File::create("graph.dot")
        file.write_all(dot_output.as_bytes())?

        println!("arquivo grvado com sucesso")
        Ok(())
}