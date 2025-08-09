// Define a service simulator data model
pub struct ServiceSimulator {
    id: i32,
    name: String,
    description: String,
    api_endpoints: Vec<ApiEndpoint>,
    data_sets: Vec<DataSet>,
}

// Define an API endpoint data model
pub struct ApiEndpoint {
    id: i32,
    method: String, // GET, POST, PUT, DELETE
    path: String,
    responseCodes: Vec<i32>,
}

// Define a data set data model
pub struct DataSet {
    id: i32,
    data_type: String, // JSON, XML, CSV
    data: Vec<String>,
}

// Define a simulator configuration data model
pub struct SimulatorConfig {
    delay: i32, // in milliseconds
    error_rate: f64, // 0.0 to 1.0
    max_responses: i32,
}

impl ServiceSimulator {
    pub fn new(id: i32, name: &str, description: &str) -> Self {
        ServiceSimulator {
            id,
            name: name.to_string(),
            description: description.to_string(),
            api_endpoints: vec![],
            data_sets: vec![],
        }
    }

    pub fn add_api_endpoint(&mut self, api_endpoint: ApiEndpoint) {
        self.api_endpoints.push(api_endpoint);
    }

    pub fn add_data_set(&mut self, data_set: DataSet) {
        self.data_sets.push(data_set);
    }

    pub fn set_simulator_config(&mut self, config: SimulatorConfig) {
        // TO DO: implement simulator configuration
    }
}