#[path = "../repository/employee_repo.rs"] mod employee_repo;


use employee_repo::*;
use models::*;

use warp::{Filter, Rejection, Reply};

pub fn create_employee_endpoints() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let employees = warp::path("employees");

    
    let get_all = employees
        .and(warp::get())
        .and(warp::path::end())
        .map(|| warp::reply::json(&get_all_employees()));
    
    let get_single = employees
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .map(|e_id: i32| warp::reply::json(&get_employee(e_id)));
    
    let insert = employees
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|employee: NewEmployee| warp::reply::json(&insert_employee(employee)));
    
    let put = employees
        .and(warp::put())
        .and(warp::path::end())
        .and(warp::body::json())
        .map(| employee: Employee | warp::reply::json(&update_employee(employee)));
    
    let delete = employees
        .and(warp::delete())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .map(|e_id: i32| warp::reply::json(&delete_employee(e_id)));

    return get_all.or(get_single).or(insert).or(put).or(delete)
}
