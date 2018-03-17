use websocket::client::ClientBuilder;

use receiver::Receiver;
use sender::Sender;
use error::ConnectError;


const PHOENIX_VERSION: &str = "2.0.0";


pub fn connect(url: &str, params: Vec<(&str, &str)>) -> Result<(Sender, Receiver), ConnectError> {
    // convert the params to a uri component string
    let mut params_uri: String = "".to_owned();
    for (k, v) in params {
        params_uri.push_str(&format!("&{}={}", k, v));
    }

    // create a phoenix socket url with params expanded and parse it
    // phoenix socket endpoints always have /websocket appended for the socket route
    // it also adds the vsn parameter for versioning
    let addr = format!("{}/websocket?&vsn={}{}", url, PHOENIX_VERSION, params_uri);
    let mut client_builder = ClientBuilder::new(&addr)?;

    let socket_client = client_builder.connect_insecure()?;
    let (reader, writer) = socket_client.split()?;

    let sender = Sender::new(writer);
    let receiver = Receiver::new(reader);

    return Ok((sender, receiver));
}
