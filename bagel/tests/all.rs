use bagel::Constdef;

#[derive(Constdef)]
struct Port {
    requests: usize,
    admin: bool,
}

#[derive(Constdef)]
struct PortLogger {
    ports: [Port; 65536],
    root_pid: usize,
}

const PORT_LOGGER: PortLogger = PortLogger::default();

#[test]
fn nested_object() {
    assert_eq!(PORT_LOGGER.ports[0].requests, 0);
    assert_eq!(PORT_LOGGER.ports[65535].admin, false);
    assert_eq!(PORT_LOGGER.root_pid, 0);
}
