use super::service;

pub struct  AppController{
  app_service: service::AppService
}

impl AppController {
  pub fn new(app_service: service::AppService) -> AppController {
    AppController{
      app_service
    }
  }
  
  fn get_hello_world(&self)-> String{
    self.app_service.get_hello_world()
  }
}

impl nestrs::Controller for AppController {
}