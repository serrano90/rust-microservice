/**
 * Resort Service Contract
 */

pub trait ResortService {
    fn validate(hotel_id: String) -> bool;
} 