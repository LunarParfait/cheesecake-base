/// # Create Routes
///
/// Helper macro to create routes for the application
/// using a custom DSL.
#[macro_export]
macro_rules! create_routes {

    // Nesting routes at root
    // route ["/"?] => [router]
    (@parse $router:ident route $("/")? => $inner:expr, $($rest:tt)*) => {
        let $router = $router.merge($inner);
        create_routes!(@parse $router $($rest)*);
    };

    // Nesting routes
    // route [path] => [router]
    (@parse $router:ident route $path:literal => $inner:expr, $($rest:tt)*) => {
        let $router = $router.nest($path, $inner);
        create_routes!(@parse $router $($rest)*);
    };

    // Creating simple routes
    // [method] [path] => [handler]
    (@parse $router:ident $method:ident $path:literal => $handler:expr, $($rest:tt)*) => {
        let $router = $router.route($path, $method($handler));
        create_routes!(@parse $router $($rest)*);
    };

    // Adding middleware
    // with [middleware] => [router]
    (@parse $router:ident with $middleware:expr => $inner:expr, $($rest:tt)*) => {
        let $router = $router.merge(
            Router::new()
            .layer($middleware)
            .merge($inner));
        create_routes!(@parse $router $($rest)*);
    };

    // Nesting routes with middleware
    // route [path] with [middleware] => [router]
    (@parse $router:ident route $path:literal with $middleware:expr => $inner:expr, $($rest:tt)*) => {
        let $router = $router.nest($path,
            Router::new()
            .layer($middleware)
            .merge($inner));
        create_routes!(@parse $router $($rest)*);
    };

    // No more tokens left
    (@parse $router:ident) => {};

    // Error during last statement without comma
    (@parse $router:ident $($stm:tt)*) => {
        compile_error!(stringify!(Invalid syntax while parsing statement: $($stm)*));
    };

    // Error during some statement with comma
    (@parse $router:ident $($stm:tt)*, $($tts:tt)*) => {
        compile_error!(stringify!(Invalid syntax while parsing statement: $($stm)*));
    };

    // Initial macro invocation
    ($($tts:tt)*) => {{
        let router = Router::new();
        create_routes!(@parse router $($tts)*);

        router
    }};

}
