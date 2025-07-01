use anyhow::Result;
use std::collections::HashMap;

/// Represents a geographic coordinate
#[derive(Debug, Clone)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

/// Calculate distance between two coordinates using Haversine formula
pub fn calculate_distance(coord1: &Coordinate, coord2: &Coordinate) -> f64 {
    let earth_radius_km = 6371.0;
    
    let lat1_rad = coord1.latitude.to_radians();
    let lat2_rad = coord2.latitude.to_radians();
    let delta_lat = (coord2.latitude - coord1.latitude).to_radians();
    let delta_lon = (coord2.longitude - coord1.longitude).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius_km * c
}

/// Get geographic risk score based on location
pub fn get_location_risk_score(ip_address: &str) -> Result<f64> {
    // Implementation would use IP geolocation service
    // For now, return a placeholder score
    Ok(match ip_address {
        ip if ip.starts_with("127.") => 0.0, // Local
        ip if ip.starts_with("192.168.") => 0.0, // Private network
        ip if ip.starts_with("10.") => 0.0, // Private network
        _ => 20.0, // Default risk score for external IPs
    })
}

/// Check if location is in a high-risk country/region
pub fn is_high_risk_location(country_code: &str) -> bool {
    // Simplified high-risk country list (should be configurable)
    let high_risk_countries = ["XX", "YY", "ZZ"]; // Placeholder codes
    high_risk_countries.contains(&country_code)
}

/// Calculate velocity risk based on geographic movement
pub fn calculate_velocity_risk(
    previous_location: &Coordinate,
    current_location: &Coordinate,
    time_diff_hours: f64,
) -> f64 {
    if time_diff_hours <= 0.0 {
        return 0.0;
    }

    let distance_km = calculate_distance(previous_location, current_location);
    let velocity_kmh = distance_km / time_diff_hours;
    
    // Risk scoring based on impossible travel speeds
    match velocity_kmh {
        v if v > 1000.0 => 100.0, // Impossible by commercial aircraft
        v if v > 800.0 => 80.0,   // Very unlikely by aircraft
        v if v > 200.0 => 50.0,   // Unlikely by ground transport
        v if v > 100.0 => 20.0,   // Fast but possible
        _ => 0.0,                 // Normal travel speed
    }
}

/// Analyze transaction pattern by geographic clustering
pub fn analyze_geographic_clusters(
    transactions: &[(Coordinate, chrono::DateTime<chrono::Utc>)],
) -> HashMap<String, f64> {
    let mut features = HashMap::new();
    
    if transactions.len() < 2 {
        return features;
    }

    // Calculate geographic dispersion
    let mut total_distance = 0.0;
    for i in 1..transactions.len() {
        total_distance += calculate_distance(&transactions[i-1].0, &transactions[i].0);
    }
    
    let avg_distance = total_distance / (transactions.len() - 1) as f64;
    features.insert("avg_geographic_distance".to_string(), avg_distance);
    features.insert("total_geographic_spread".to_string(), total_distance);
    
    // Calculate unique locations (simplified clustering)
    let unique_locations = estimate_unique_locations(transactions);
    features.insert("unique_location_count".to_string(), unique_locations as f64);
    
    features
}

/// Estimate number of unique locations (simplified clustering)
fn estimate_unique_locations(
    transactions: &[(Coordinate, chrono::DateTime<chrono::Utc>)],
) -> usize {
    let cluster_threshold_km = 10.0; // Consider locations within 10km as same location
    let mut clusters = Vec::new();
    
    for (coord, _) in transactions {
        let mut found_cluster = false;
        for cluster_coord in &clusters {
            if calculate_distance(coord, cluster_coord) < cluster_threshold_km {
                found_cluster = true;
                break;
            }
        }
        if !found_cluster {
            clusters.push(coord.clone());
        }
    }
    
    clusters.len()
} 