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

pub struct UserApiImpl<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> UserApiImpl<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UserApiImpl<C> {
        UserApiImpl {
            configuration: configuration,
        }
    }
}

pub trait UserApi {
    fn CreateUser(&self, body: User) -> Box<Future<Item = (), Error = Error>>;
    fn CreateUsersWithArrayInput(&self, body: Vec<User>) -> Box<Future<Item = (), Error = Error>>;
    fn CreateUsersWithListInput(&self, body: Vec<User>) -> Box<Future<Item = (), Error = Error>>;
    fn DeleteUser(&self, username: String) -> Box<Future<Item = (), Error = Error>>;
    fn GetUserByName(&self, username: String) -> Box<Future<Item = User, Error = Error>>;
    fn LoginUser(&self, username: String, password: String) -> Box<Future<Item = String, Error = Error>>;
    fn LogoutUser(&self, ) -> Box<Future<Item = (), Error = Error>>;
    fn UpdateUser(&self, username: String, body: User) -> Box<Future<Item = (), Error = Error>>;
}


impl<C: hyper::client::Connect>UserApi for UserApiImpl<C> {
    fn CreateUser(&self, body: User) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("{}/user", configuration.base_path, body=body));

        // body params
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

    fn CreateUsersWithArrayInput(&self, body: Vec<User>) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("{}/user/createWithArray", configuration.base_path, body=body));

        // body params
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

    fn CreateUsersWithListInput(&self, body: Vec<User>) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Post,
            format!("{}/user/createWithList", configuration.base_path, body=body));

        // body params
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

    fn DeleteUser(&self, username: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Delete,
            format!("{}/user/{username}", configuration.base_path, username=username));

    }

    fn GetUserByName(&self, username: &str) -> Box<Future<Item = super::User, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("{}/user/{username}", configuration.base_path, username=username));

    }

    fn LoginUser(&self, username: &str, password: &str) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("{}/user/login", configuration.base_path, username=username, password=password));

        // TODO query parameter username(username)
        // TODO query parameter password(password)
    }

    fn LogoutUser(&self, ) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Get,
            format!("{}/user/logout", configuration.base_path));

    }

    fn UpdateUser(&self, username: &str, body: User) -> Box<Future<Item = (), Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();
        let mut req = hyper::Request::new(
            hyper::Method::Put,
            format!("{}/user/{username}", configuration.base_path, username=username, body=body));

        // body params
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

}