use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    vocab_file: PathBuf,
    mastered_file: PathBuf,
    unmastered_file: PathBuf,
    remaining_words: VecDeque<String>,
    history: Vec<Action>,
}

#[derive(Clone)]
struct Action {
    card: String,
    is_mastered: bool,
}

#[derive(Serialize)]
struct Stats {
    mastered: usize,
    unmastered: usize,
    remaining: usize,
    total: usize,
}

#[derive(Serialize)]
struct Word {
    card: String,
    word: String,
    meaning: String,
}

#[derive(Deserialize)]
struct ClassifyReq {
    status: String,
}

#[derive(Deserialize)]
struct AddWordRequest {
    card: Option<String>,
    word: Option<String>,
    meaning: Option<String>,
}

impl AppState {
    fn new(vocab_file: PathBuf) -> Self {
        Self {
            vocab_file: vocab_file.clone(),
            mastered_file: PathBuf::from("mastered_words.txt"),
            unmastered_file: PathBuf::from("unmastered_words.txt"),
            remaining_words: VecDeque::new(),
            history: Vec::new(),
        }
    }

    fn load_vocabulary(&mut self) -> std::io::Result<()> {
        let file = fs::File::open(&self.vocab_file)?;
        let reader = BufReader::new(file);
        self.remaining_words = reader
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| !l.trim().is_empty())
            .collect();
        Ok(())
    }

    fn parse_word(line: &str) -> (String, String) {
        if let Some(idx) = line.find('\t') {
            let (w, m) = line.split_at(idx);
            (w.trim().to_string(), m[1..].trim().to_string())
        } else {
            (line.trim().to_string(), String::new())
        }
    }

    fn update_vocab_file(&self) -> std::io::Result<()> {
        let mut file = fs::File::create(&self.vocab_file)?;
        for w in &self.remaining_words {
            writeln!(file, "{}", w)?;
        }
        Ok(())
    }

    fn append_word(&self, word: &str, mastered: bool) -> std::io::Result<()> {
        let path = if mastered {
            &self.mastered_file
        } else {
            &self.unmastered_file
        };
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", word)?;
        Ok(())
    }

    fn remove_last_line(path: &PathBuf) -> std::io::Result<()> {
        if !path.exists() {
            return Ok(());
        }
        let content = fs::read_to_string(path)?;
        let mut lines: Vec<&str> = content.lines().collect();
        if !lines.is_empty() {
            lines.pop();
        }
        fs::write(path, lines.join("\n"))?;
        Ok(())
    }

    fn count_lines(path: &PathBuf) -> usize {
        fs::File::open(path)
            .ok()
            .map(|f| {
                BufReader::new(f)
                    .lines()
                    .filter(|l| l.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false))
                    .count()
            })
            .unwrap_or(0)
    }
}

type SharedState = Arc<Mutex<AppState>>;

async fn get_stats(State(state): State<SharedState>) -> Json<Stats> {
    let st = state.lock().await;
    let mastered = AppState::count_lines(&st.mastered_file);
    let unmastered = AppState::count_lines(&st.unmastered_file);
    let remaining = st.remaining_words.len();
    Json(Stats {
        mastered,
        unmastered,
        remaining,
        total: mastered + unmastered + remaining,
    })
}

async fn get_word(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let st = state.lock().await;
    if let Some(line) = st.remaining_words.front() {
        let (w, m) = AppState::parse_word(line);
        Json(serde_json::json!({ "card": line, "word": w, "meaning": m }))
    } else {
        Json(serde_json::json!({}))
    }
}

async fn classify(State(state): State<SharedState>, Json(req): Json<ClassifyReq>) -> Json<serde_json::Value> {
    let mut st = state.lock().await;
    if let Some(card) = st.remaining_words.pop_front() {
        let mastered = req.status.to_lowercase() == "mastered";
        if st.append_word(&card, mastered).is_ok() {
            st.history.push(Action { card: card.clone(), is_mastered: mastered });
            let _ = st.update_vocab_file();
        }
        Json(serde_json::json!({ "message": "ok" }))
    } else {
        Json(serde_json::json!({ "error": "no word" }))
    }
}

async fn skip(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let mut st = state.lock().await;
    if let Some(card) = st.remaining_words.pop_front() {
        st.remaining_words.push_back(card);
        let _ = st.update_vocab_file();
    }
    Json(serde_json::json!({ "message": "skipped" }))
}

async fn undo(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let mut st = state.lock().await;
    if let Some(action) = st.history.pop() {
        if action.is_mastered {
            let _ = AppState::remove_last_line(&st.mastered_file);
        } else {
            let _ = AppState::remove_last_line(&st.unmastered_file);
        }
        st.remaining_words.push_front(action.card.clone());
        let _ = st.update_vocab_file();
        Json(serde_json::json!({ "message": "undone" }))
    } else {
        Json(serde_json::json!({ "error": "nothing to undo" }))
    }
}

async fn shuffle(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let mut st = state.lock().await;
    let mut rng = rand::thread_rng();
    st.remaining_words.make_contiguous().shuffle(&mut rng);
    let _ = st.update_vocab_file();
    Json(serde_json::json!({ "message": "shuffled" }))
}

async fn reset(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let mut st = state.lock().await;
    let mastered = fs::read_to_string(&st.mastered_file).unwrap_or_default();
    let unmastered = fs::read_to_string(&st.unmastered_file).unwrap_or_default();
    for line in mastered.lines().chain(unmastered.lines()) {
        if !line.trim().is_empty() {
            st.remaining_words.push_back(line.to_string());
        }
    }
    let _ = st.update_vocab_file();
    let _ = fs::write(&st.mastered_file, "");
    let _ = fs::write(&st.unmastered_file, "");
    st.history.clear();
    Json(serde_json::json!({ "message": "reset" }))
}

async fn add_word(State(state): State<SharedState>, Json(req): Json<AddWordRequest>) -> Json<Word> {
    let mut st = state.lock().await;
    let card = req.card.unwrap_or_default();
    let word = req.word.unwrap_or_default();
    let meaning = req.meaning.unwrap_or_default();
    let line = if !card.trim().is_empty() {
        card.trim().to_string()
    } else if !meaning.trim().is_empty() {
        format!("{}\t{}", word.trim(), meaning.trim())
    } else {
        word.trim().to_string()
    };
    st.remaining_words.push_back(line.clone());
    let _ = st.update_vocab_file();
    let (w, m) = AppState::parse_word(&line);
    Json(Word { card: line, word: w, meaning: m })
}

#[tokio::main]
async fn main() {
    let mut app_state = AppState::new(PathBuf::from("vocab.txt"));
    if let Err(e) = app_state.load_vocabulary() {
        eprintln!("Failed to load vocabulary: {}", e);
    }
    let shared = Arc::new(Mutex::new(app_state));

    let app = Router::new()
        .route("/api/stats", get(get_stats))
        .route("/api/word", get(get_word))
        .route("/api/classify", post(classify))
        .route("/api/skip", post(skip))
        .route("/api/undo", post(undo))
        .route("/api/shuffle", post(shuffle))
        .route("/api/reset", post(reset))
        .route("/api/word", post(add_word))
        .with_state(shared)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    println!("Starting server on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
