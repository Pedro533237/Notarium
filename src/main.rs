use notarium::NotariumApp;

fn main() {
    let app = NotariumApp::bootstrap_for_legacy_hardware();
    println!(
        "Notarium iniciado em modo legado: backend={:?}, low_performance={}.",
        app.audio_engine.backend(),
        app.audio_engine.config().low_performance_mode
    );
}
