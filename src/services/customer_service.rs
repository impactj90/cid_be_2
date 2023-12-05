use actix_web::web;

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        customer::{Customer, CustomerDTO}
    },
};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Customer>, ServiceError> {
    match Customer::find_all(&mut pool.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        }),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Customer, ServiceError> {
    match Customer::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Customer with id {} not found", id),
        }),
    }
}

pub fn insert(new_person: CustomerDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Customer::insert(new_person, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        }),
    }
}

pub fn update(
    id: i32,
    updated_person: CustomerDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Customer::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Customer::update(id, updated_person, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            }),
        },
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Customer with id {} not found", id),
        }),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Customer::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Customer::delete(id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            }),
        },
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Customer with id {} not found", id),
        }),
    }
}
