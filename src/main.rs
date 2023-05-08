use std::collections::HashMap;
use std::error::Error;
use std::str::Split;
use csv;
use serde::Deserialize;
use std::env;

use graphrs::{algorithms::centrality::degree,  Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::centrality::closeness, ErrorKind};
use graphrs::{algorithms::centrality::betweenness};
use graphrs::{algorithms::centrality::eigenvector};


#[derive(Debug, Deserialize)]
struct ReadAirport {
    Origin_airport: String,
    Destination_airport: String,
    Numtimes: String,
    Passengers: String,
    Seats: String ,
    Flights: String,
    Distance: String,
    Origin_population: String,
    Destination_population: String
}


#[derive(Debug)]
struct Airport {
    Origin_airport: String,
    Destination_airport: String,
    Numtimes: f64,
    Passengers: f64,
    Seats: f64 ,
    Flights: f64,
    Distance: f64,
    Origin_population: f64,
    Destination_population: f64,
}

/* 
 Usage: cargo run -- airport.txt {Numtimes | Passengers | Seats | Flights | Distance} */

fn main() -> Result<(), Box<dyn Error>> {
    /*println!("Hello, world!");*/
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let query_arg = &args[2];

    println!("Using data file:  {}", file_path);
    /*println!("Searching for {}", query_arg);*/


    let mut reader = csv::Reader::from_path(file_path)?;

    let mut v_airport : Vec<Airport> = Vec::new();

    // Retrieve and print header record
    let headers = reader.headers()?;
    /* println!("{:?}", headers); */

    for result in reader.deserialize() {
        let record: ReadAirport = result?;

        let o_Numtimes: f64 = record.Numtimes.parse().unwrap();
        let o_Passengers: f64 = record.Passengers.parse().unwrap();
        let o_Seats: f64 = record.Seats.parse().unwrap();
        let o_Flights: f64 = record.Flights.parse().unwrap();
        let o_Distance: f64 = record.Distance.parse().unwrap();                        
        let o_Origin_population: f64 = record.Origin_population.parse().unwrap();
        let o_Destination_population: f64 = record.Destination_population.parse().unwrap();                        
        let mut oneairport = Airport 
            {
                Origin_airport: record.Origin_airport,
                Destination_airport: record.Destination_airport,
                Numtimes: o_Numtimes,
                Passengers: o_Passengers,
                Seats: o_Seats ,
                Flights: o_Flights,
                Distance: o_Distance,
                Origin_population: o_Origin_population,
                Destination_population: o_Destination_population,
            };
        v_airport.push(oneairport);                        
    }

    /* create a hashmap with unique airport codes and population */
    let mut airport_hash:HashMap<String, f64> = HashMap::new();
    /*println!("vector-length={}",v_airport.len());*/
    for x in 0..v_airport.len() {
        let atmp = v_airport[x].Origin_airport.clone();        
        let btmp = v_airport[x].Destination_airport.clone();
        /*println!("{} {} {:?} ", v_airport[x].Origin_airport, v_airport[x].Destination_airport, v_airport[x].Distance);*/
        if airport_hash.contains_key(&atmp) {
            /* println!("in map already");*/
        }
        else {
            /* println!("NOT in map already");    */                    
            airport_hash.insert(atmp,v_airport[x].Origin_population);
        }
        if airport_hash.contains_key(&btmp) {
            /*println!("in map already");*/
        }
        else {
            /* println!("NOT in map already");  */                      
            airport_hash.insert(btmp,v_airport[x].Destination_population);
        }
    }


    /*let mut graph = get_basic_graph(None);*/
    /*let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed());*/
    let mut graph: Graph<&str, f64> = Graph::new(GraphSpecs::undirected());


    /* this is done just to make sure we get unique airport codes */
    for (key, value) in &airport_hash {
        /*println!("{} : {}", key, value);*/
        let airportcode = key.as_str().clone();
        /*let def = key.as_str().clone();        */
        /*graph.add_node(Node::from_name("n1"));*/
        graph.add_node(Node::from_name_and_attributes(airportcode,*value));
    }


    println!("Number of nodes/unique airport codes={}",graph.get_all_nodes().len());

    /* Add all the origin / destination airport code as an edge */
    for x in 0..v_airport.len() {
        let atmp = v_airport[x].Origin_airport.as_str().clone(); 
        /*let atmp1 = atmp.as_str().clone();*/
        let btmp = v_airport[x].Destination_airport.as_str().clone();
        /* let btmp1 = btmp.as_str().clone();        */
        /* println!("{:?}", v_airport[x]); */
        if query_arg.eq("Numtimes") {
            graph.add_edge(Edge::with_weight(atmp,btmp, v_airport[x].Numtimes));                    
        }
        else if query_arg.eq("Passengers") {
            graph.add_edge(Edge::with_weight(atmp,btmp, v_airport[x].Passengers));                    
        }
        else if query_arg.eq("Seats") {
            graph.add_edge(Edge::with_weight(atmp,btmp, v_airport[x].Seats));                    
        }
        else if query_arg.eq("Flights") {
            graph.add_edge(Edge::with_weight(atmp,btmp, v_airport[x].Flights));                    
        }
        else if query_arg.eq("Distance") {
            graph.add_edge(Edge::with_weight(atmp,btmp, v_airport[x].Distance));                    
        }
        else  {  /* default */
            graph.add_edge(Edge::with_weight(atmp,btmp, v_airport[x].Flights));                    
        }

    }


    /*println!("graph.get_all_nodes().len()={}",graph.get_all_nodes().len());*/
    println!("Total number of flights/edges={} using weight from {}",graph.get_all_edges().len(), query_arg);
    

    let result = degree::degree_centrality(&graph);
    println!(" degree centrality for ORD ={}",result.get("ORD").unwrap());
    println!(" degree centrality for CMI ={}",result.get("CMI").unwrap());
    println!(" degree centrality for FLL ={}",result.get("FLL").unwrap());
    println!(" degree centrality for ATL ={}",result.get("ATL").unwrap());
    println!(" degree centrality for DCA ={}",result.get("DCA").unwrap());
    println!(" degree centrality for PDX ={}",result.get("PDX").unwrap());
    /* test_add_nodes(); */
/*    test_degree_centrality_1();*/


    let result = closeness::closeness_centrality(&graph, true, true).unwrap();
    println!(" closeness centrality for ORD ={}",result.get("ORD").unwrap());
    println!(" closeness centrality for CMI ={}",result.get("CMI").unwrap());
    println!(" closeness centrality for FLL ={}",result.get("FLL").unwrap());
    println!(" closeness centrality for ATL ={}",result.get("ATL").unwrap());
    println!(" closeness centrality for DCA ={}",result.get("DCA").unwrap());
    println!(" closeness centrality for PDX ={}",result.get("PDX").unwrap());


    let result = betweenness::betweenness_centrality(&graph, true, false).unwrap();
    println!(" betweenness centrality for ORD ={}",result.get("ORD").unwrap());
    println!(" betweenness centrality for CMI ={}",result.get("CMI").unwrap());
    println!(" betweenness centrality for FLL ={}",result.get("FLL").unwrap());
    println!(" betweenness centrality for ATL ={}",result.get("ATL").unwrap());
    println!(" betweenness centrality for DCA ={}",result.get("DCA").unwrap());
    println!(" betweenness centrality for PDX ={}",result.get("PDX").unwrap());


    let result = eigenvector::eigenvector_centrality(&graph, true, None, None).unwrap();
    println!(" eigenvector centrality for ORD ={}",result.get("ORD").unwrap());
    println!(" eigenvector centrality for CMI ={}",result.get("CMI").unwrap());
    println!(" eigenvector centrality for FLL ={}",result.get("FLL").unwrap());
    println!(" eigenvector centrality for ATL ={}",result.get("ATL").unwrap());
    println!(" eigenvector centrality for DCA ={}",result.get("DCA").unwrap());
    println!(" eigenvector centrality for PDX ={}",result.get("PDX").unwrap());





    Ok(())
}
