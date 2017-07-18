/* 
 * Swagger Petstore
 *
 * This is a sample server Petstore server.  You can find out more about Swagger at [http://swagger.io](http://swagger.io) or on [irc.freenode.net, #swagger](http://swagger.io/irc/).  For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * Contact: apiteam@swagger.io
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration, models};

pub struct PetApiImpl<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PetApiImpl<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PetApiImpl<C> {
        PetApiImpl {
            configuration: configuration,
        }
    }
}

pub trait PetApi {
    fn AddPet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>>;
    fn DeletePet(&self, pet_id: i64, api_key: String) -> Box<Future<Item = (), Error = Error>>;
    fn FindPetsByStatus(&self, status: Vec<String>) -> Box<Future<Item = Vec<Pet>, Error = Error>>;
    fn FindPetsByTags(&self, tags: Vec<String>) -> Box<Future<Item = Vec<Pet>, Error = Error>>;
    fn GetPetById(&self, pet_id: i64) -> Box<Future<Item = Pet, Error = Error>>;
    fn UpdatePet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>>;
    fn UpdatePetWithForm(&self, pet_id: i64, name: String, status: String) -> Box<Future<Item = (), Error = Error>>;
    fn UploadFile(&self, pet_id: i64, additional_metadata: String, file: File) -> Box<Future<Item = ApiResponse, Error = Error>>;
}


impl<C: hyper::client::Connect>PetApi for PetApiImpl<C> {
    fn AddPet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("/pet", configuration.base_path).parse().unwrap());

        /// body params
        let serialized = serde_json::to_string(body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn DeletePet(&self, pet_id: i64api_key: String) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Delete,
            format!("/pet/{petId}", configuration.base_path).parse().unwrap());

        /// TODO header parameter api_key(api_key) Optional
    }

    fn FindPetsByStatus(&self, status: Vec<String>) -> Box<Future<Item = Vec<Pet>, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("/pet/findByStatus", configuration.base_path).parse().unwrap());

        /// TODO query parameter status(status)
    }

    fn FindPetsByTags(&self, tags: Vec<String>) -> Box<Future<Item = Vec<Pet>, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("/pet/findByTags", configuration.base_path).parse().unwrap());

        /// TODO query parameter tags(tags)
    }

    fn GetPetById(&self, pet_id: i64) -> Box<Future<Item = Pet, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("/pet/{petId}", configuration.base_path).parse().unwrap());

    }

    fn UpdatePet(&self, body: Pet) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Put,
            format!("/pet", configuration.base_path).parse().unwrap());

        /// body params
        let serialized = serde_json::to_string(body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn UpdatePetWithForm(&self, pet_id: i64name: Stringstatus: String) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("/pet/{petId}", configuration.base_path).parse().unwrap());

        /// TODO form parameter name(name) Optional
        /// TODO form parameter status(status) Optional
    }

    fn UploadFile(&self, pet_id: i64additional_metadata: Stringfile: File) -> Box<Future<Item = ApiResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("/pet/{petId}/uploadImage", configuration.base_path).parse().unwrap());

        /// TODO form parameter additionalMetadata(additional_metadata) Optional
        /// TODO form parameter (file) file(file) Optional
    }

}
