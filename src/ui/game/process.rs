use actix_web::{web, HttpResponse, Result, HttpRequest};
use core::{FootballSimulator};
use crate::GameAppData;
use actix_web::web::Data;
use std::sync::Arc;
use actix_web::http::header::REFERER;
use core::utils::TimeEstimation;

pub async fn game_process_action(request: HttpRequest, state: Data<GameAppData>) -> Result<HttpResponse> {
    let mut data = state.data.lock().unwrap();
    
    let simulator_data = data.as_mut().unwrap();
    
    let (_, estimated) = TimeEstimation::estimate(|| FootballSimulator::simulate(simulator_data));   
    let referrer = request.headers().get(REFERER).unwrap();
    
    Ok(HttpResponse::Found()
        .header("Location", referrer.to_str().unwrap_or("/"))
        .header("Estimated", estimated.to_string())
        .finish())
}