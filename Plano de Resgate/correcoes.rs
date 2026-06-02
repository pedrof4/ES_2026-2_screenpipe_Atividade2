// Interface abstrata para os motores de processamento de mídia
trait MediaProcessorFactory {
    fn create_ocr_engine(&self) -> Box<dyn OcrEngine>;
    fn create_audio_transcriber(&self) -> Box<dyn AudioTranscriber>;
}

// Interfaces dos produtos
trait OcrEngine {
    fn extract_text(&self, frame: &[u8]) -> String;
}
trait AudioTranscriber {
    fn transcribe(&self, audio_data: &[u8]) -> String;
}

// Fábrica Concreta para Ambientes Apple (Local-First otimizado)
struct AppleIntelligenceFactory;
impl MediaProcessorFactory for AppleIntelligenceFactory {
    fn create_ocr_engine(&self) -> Box<dyn OcrEngine> {
        Box::new(AppleVisionOcr::new())
    }
    fn create_audio_transcriber(&self) -> Box<dyn AudioTranscriber> {
        Box::new(LocalWhisperTranscriber::new())
    }
}

// Fábrica Concreta para Ambientes Linux/Windows Cloud-Fallback
struct CloudFallbackFactory;
impl MediaProcessorFactory for CloudFallbackFactory {
    fn create_ocr_engine(&self) -> Box<dyn OcrEngine> {
        Box::new(TesseractOcr::new())
    }
    fn create_audio_transcriber(&self) -> Box<dyn AudioTranscriber> {
        Box::new(DeepgramCloudTranscriber::new())
    }
}

// Gerenciador do Core de Captura (Desacoplado)
struct CaptureEngine {
    ocr_processor: Box<dyn OcrEngine>,
    audio_processor: Box<dyn AudioTranscriber>,
}

impl CaptureEngine {
    fn new(factory: &dyn MediaProcessorFactory) -> Self {
        CaptureEngine {
            ocr_processor: factory.create_ocr_engine(),
            audio_processor: factory.create_audio_transcriber(),
        }
    }
    
    fn process_loop(&self, frame: &[u8], audio: &[u8]) {
        let text = self.ocr_processor.extract_text(frame);
        let audio_text = self.audio_processor.transcribe(audio);
        // Persiste no SQLite local...
    }
}
