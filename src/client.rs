extern crate hyper;



/// Creates a method with a supplied name, HTTP verb, and resource path.
macro_rules! create_api_method {
    ($func_name:ident, $verb:ident, $resource:ty) => {
        fn $func_name<T>(&mut self, body: &T) -> Result<client::response::Response, Error> {
            let url = self.make_api_uri($resource);
            let result = try!(self.http
                              .$verb(&url)
                              .headers(self.headers.clone())
                              .send());
        }
    }
}

