use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub input: String,
    #[serde(rename = "guessLanguage")]
    pub guess_language: String,
    #[serde(rename = "isHasSimpleDict")]
    pub is_has_simple_dict: String,
    pub le: String,
    pub lang: String,
    pub dicts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippets {
    pub snippet: Value,
}
/// 原声例句
#[derive(Serialize, Deserialize, Debug)]
pub struct MediaSentsPart {
    #[serde(rename = "sentence-count", default)]
    pub sentence_count: i64,
    #[serde(default)]
    pub more: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sent {
    pub score: f64,
    pub speech: String,
    #[serde(rename = "speech-size")]
    pub speech_size: String,
    pub source: String,
    pub url: String,
    pub foreign: String,
}
/// 权威例句
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthSentsPart {
    #[serde(rename = "sentence-count", default)]
    pub sentence_count: i64,
    #[serde(default)]
    pub more: String,
    pub sent: Vec<Sent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordRel {
    pub word: String,
    pub tran: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelDetail {
    pub pos: String,
    pub words: Vec<WordRel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rel {
    pub rel: RelDetail,
}

/// 同根词
#[derive(Serialize, Deserialize, Debug)]
pub struct RelWord {
    /// 词根
    pub word: String,
    pub stem: String,
    #[serde(default)]
    pub rels: Vec<Rel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sense {
    pub lang: String,
    pub word: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Words {
    pub indexforms: Vec<String>,
    pub word: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollinsPrimary {
    pub words: Words,
    pub gramcat: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PastExamSent {
    pub en: String,
    pub source: String,
    pub zh: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Individual {
    #[serde(rename = "pastExamSents", default)]
    pub past_exam_sents: Vec<PastExamSent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentencePair {
    /// 例句
    pub sentence: String,
    #[serde(rename = "sentence-eng")]
    pub sentence_eng: String,
    /// 例句翻译
    #[serde(rename = "sentence-translation")]
    pub sentence_translation: String,
    #[serde(default)]
    pub source: String,
    pub url: String,
    #[serde(rename = "sentence-speech")]
    pub sentence_speech: String,
}
/// 双语例句
#[derive(Serialize, Deserialize, Debug)]
pub struct BlngSentsPart {
    #[serde(rename = "sentence-count", default)]
    pub sentence_count: i64,
    #[serde(rename = "sentence-pair")]
    pub sentence_pair: Vec<SentencePair>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EeWordTr {
    pub pos: String,
    pub tr: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EeWord {
    pub trs: Vec<EeWordTr>,
    #[serde(default)]
    pub phone: String,
    pub speech: String,
    #[serde(rename = "return-phrase")]
    pub return_phrase: String,
}
/// 英文释义
#[derive(Serialize, Deserialize, Debug)]
pub struct Ee {
    pub source: Source,
    pub word: EeWord,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WfDetail {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wf {
    pub wf: WfDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tr {
    pub pos: Option<String>,
    pub tran: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    #[serde(default)]
    pub usphone: String,
    #[serde(default)]
    pub ukphone: String,
    #[serde(default)]
    pub ukspeech: String,
    #[serde(default)]
    pub trs: Vec<Tr>,
    #[serde(default)]
    pub wfs: Vec<Wf>,
    #[serde(rename = "return-phrase")]
    pub return_phrase: String,
    #[serde(default)]
    pub usspeech: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub name: String,
    pub url: String,
}

/// 中文释义
#[derive(Serialize, Deserialize, Debug)]
pub struct Ec {
    #[serde(default)]
    pub web_trans: Vec<String>,
    #[serde(default)]
    pub special: Vec<Value>,
    #[serde(default)]
    pub exam_type: Vec<String>,
    pub source: Source,
    pub word: Word,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Discriminate {
    pub data: Vec<Value>,
    #[serde(rename = "return-phrase")]
    pub return_phrase: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SynoDetail {
    pub pos: String,
    pub ws: Vec<String>,
    pub tran: String,
}

/// 同近义词
#[derive(Serialize, Deserialize, Debug)]
pub struct Syno {
    pub synos: Vec<SynoDetail>,
    pub word: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phr {
    pub headword: String,
    pub translation: String,
}

/// 词组短语
#[derive(Serialize, Deserialize, Debug)]
pub struct Phrs {
    pub word: String,
    #[serde(default)]
    pub phrs: Vec<Phr>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleWord {
    #[serde(default)]
    pub usphone: String,
    #[serde(default)]
    pub ukphone: String,
    #[serde(default)]
    pub ukspeech: String,
    #[serde(rename = "return-phrase")]
    pub return_phrase: String,
    #[serde(default)]
    pub usspeech: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Simple {
    pub query: String,
    pub word: Vec<SimpleWord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordInfo {
    #[serde(rename = "return-phrase")]
    pub return_phrase: String,
    pub sense: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoSents {
    pub sents_data: Vec<Value>,
    pub word_info: WordInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordAllInfo {
    pub video_sents: Option<VideoSents>,
    pub simple: Option<Simple>,
    pub phrs: Option<Phrs>,
    pub syno: Option<Syno>,
    pub discriminate: Option<Discriminate>,
    pub lang: String,
    pub ec: Ec,
    pub ee: Option<Ee>,
    pub blng_sents_part: Option<BlngSentsPart>,
    pub individual: Option<Individual>,
    pub collins_primary: Option<CollinsPrimary>,
    pub rel_word: Option<RelWord>,
    pub auth_sents_part: Option<AuthSentsPart>,
    pub media_sents_part: Option<MediaSentsPart>,
    pub input: String,
    pub meta: Meta,
    pub le: String,
}
