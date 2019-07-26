use std::io::{ BufReader, BufRead, Write };


/// Espera a que el cliente envie un mensaje http y lo parsea
/// puede fallar en caso de que haya un problema al comunicarse con el cliente o que el
/// mensaje http no esté completo
#[inline]
pub fn leer_mensaje_http(stream: &std::net::TcpStream) -> Result<http::Request<()>, ()> {
	let mut mensaje = String::new();

	let mut reader = BufReader::new(stream);

	let mut linea = String::new();
	loop {
		reader.read_line(&mut linea).map_err( |_| () )?;
		mensaje += &linea;

		if linea.trim().is_empty() {
			break;
		}
		linea.clear();
	}

	let mut headers = [httparse::EMPTY_HEADER; 16];
	let mut parsed_request = httparse::Request::new(&mut headers);
	let parse_status = parsed_request.parse( mensaje.as_bytes() ).unwrap();

	if parse_status.is_partial() {
		return Err( () );
	} else {
		let owned_request = http::Request::builder()
			.method ( parsed_request.method.ok_or(())? )
			.uri    ( parsed_request.path.ok_or(())? )
			.version( http::Version::HTTP_11 )
			.body   ( () )
			.map_err( |_| () )?;

		return Ok(owned_request);
	}
}

/// Responde un 200 HTTP OK con un mensaje dado
#[inline]
pub fn responder_http_ok(stream: &mut std::net::TcpStream, mensaje: &str) {
	let respuesta = format!(
		"HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
		mensaje.len(),
		mensaje
	);

	stream.write_all( respuesta.as_bytes() ).unwrap();
}
