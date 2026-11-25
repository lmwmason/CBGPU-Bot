pub struct KbEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub content: &'static str,
    pub category: &'static str,
    pub keywords: &'static [&'static str],
    pub source: &'static str,
}

pub const KB_ENTRIES: &[KbEntry] = &[
    KbEntry {
        id: "cbgpu-overview",
        title: "CBGPU 소개",
        content: "CBGPU는 학교와 프로젝트를 이어가는 공간입니다. 학생의 일상부터 연구와 개발까지, 공개 프로젝트를 둘러보고 내 계정으로 학교생활과 작업을 이어갈 수 있습니다.",
        category: "overview",
        keywords: &["cbgu", "cbgpu", "소개", "개요", "학교", "프로젝트", "연결"],
        source: "https://cbgpu.com",
    },
    KbEntry {
        id: "cbgpu-school-life",
        title: "학교생활",
        content: "우리 학교의 시간표와 급식, 커뮤니티를 확인할 수 있습니다. 제공 기능은 학교마다 다를 수 있습니다.",
        category: "school_life",
        keywords: &["학교생활", "시간표", "급식", "커뮤니티", "학교"],
        source: "https://cbgpu.com/explore/services/school",
    },
    KbEntry {
        id: "cbgpu-development-projects",
        title: "개발과 프로젝트",
        content: "혼자 만든 작업도, 함께 만든 서비스도 계속 관리할 수 있습니다.",
        category: "development_projects",
        keywords: &["개발", "프로젝트", "작업", "관리", "협업"],
        source: "https://cbgpu.com/explore/services/projects",
    },
    KbEntry {
        id: "cbgpu-ai-research-tools",
        title: "AI와 연구 도구",
        content: "AI에게 질문하고 연구에 필요한 자료와 아이디어를 정리할 수 있습니다. AI 대화, 자료 검색, 글·코드 작업을 지원합니다.",
        category: "ai_research_tools",
        keywords: &[
            "AI",
            "연구",
            "도구",
            "대화",
            "자료 검색",
            "글",
            "코드",
            "정리",
        ],
        source: "https://cbgpu.com/explore/services/ai",
    },
    KbEntry {
        id: "cbgpu-gpu-computing",
        title: "GPU 컴퓨팅",
        content: "학교에 연결된 GPU를 예약하고 Jupyter로 작업할 수 있습니다. 학교별 이용 권한이 필요합니다. GPU 예약, 시간 연장, 승인 상태 확인, 공개 Jupyter 접속, 결과 저장이 가능합니다.",
        category: "gpu_computing",
        keywords: &["GPU", "컴퓨팅", "예약", "Jupyter", "학교", "권한", "연구"],
        source: "https://cbgpu.com/explore/services/gpu",
    },
    KbEntry {
        id: "cbgpu-public-projects",
        title: "공개 프로젝트",
        content: "직접 공개하기로 선택한 내용만 누구나 볼 수 있습니다. 기존의 비공개 작업은 자동으로 공개되지 않습니다.",
        category: "public_projects",
        keywords: &["공개", "프로젝트", "비공개", "공개 프로젝트"],
        source: "https://cbgpu.com/projects",
    },
    KbEntry {
        id: "cbgpu-public-contests",
        title: "공개 대회",
        content: "직접 공개한 대회가 이곳에 표시됩니다. 행사 개설부터 참가, 운영, 작품 제출까지 할 수 있습니다.",
        category: "public_contests",
        keywords: &["대회", "공개 대회", "참가", "운영", "작품 제출", "이벤트"],
        source: "https://cbgpu.com/explore/contests",
    },
    KbEntry {
        id: "cbgpu-getting-started",
        title: "시작하기",
        content: "처음이라면 가이드부터 시작하세요. 기능을 살펴보고, 필요한 순간에 내 계정으로 시작할 수 있습니다. 사용 가이드와 공개 커뮤니티를 이용할 수 있습니다.",
        category: "getting_started",
        keywords: &["시작", "가이드", "처음", "계정", "가입", "로그인"],
        source: "https://cbgpu.com",
    },
    KbEntry {
        id: "cbgpu-docs",
        title: "사용 가이드 (문서)",
        content: "CBGPU 기능별 사용법과 문제 해결 안내를 확인할 수 있습니다. 51개의 가이드가 로그인 없이 읽을 수 있습니다.",
        category: "docs",
        keywords: &["문서", "가이드", "사용법", "문제 해결", "도움말"],
        source: "https://docs.cbgpu.com",
    },
    KbEntry {
        id: "cbgpu-researcher",
        title: "연구·개발 공간",
        content: "GPU, 배포, 논문, 제작을 위한 연구·개발 공간입니다. 연구자와 개발자를 위한 전용 환경이 제공됩니다.",
        category: "research",
        keywords: &[
            "연구",
            "개발",
            "연구자",
            "개발자",
            "GPU",
            "배포",
            "논문",
            "제작",
        ],
        source: "https://researcher.cbgpu.com",
    },
    KbEntry {
        id: "cbgpu-materials",
        title: "자료 공유",
        content: "학습·연구 자료를 찾을 수 있습니다. 검색, 다운로드, 태그, 파일 업로드, 저장소 정보를 이용할 수 있습니다.",
        category: "materials",
        keywords: &[
            "자료",
            "공유",
            "학습",
            "연구",
            "검색",
            "다운로드",
            "태그",
            "파일",
        ],
        source: "https://cbgpu.com/materials",
    },
    KbEntry {
        id: "cbgpu-community",
        title: "커뮤니티",
        content: "공개 글과 학교·전국 이야기, 개인 메시지를 이용할 수 있습니다.",
        category: "community",
        keywords: &["커뮤니티", "글", "메시지", "학교", "전국", "소통"],
        source: "https://cbgpu.com/community",
    },
    KbEntry {
        id: "cbgpu-feed",
        title: "소식",
        content: "학교·기술 소식, Tech News, 서비스 공지를 확인할 수 있습니다.",
        category: "news",
        keywords: &["소식", "학교", "기술", "공지", "뉴스", "업데이트"],
        source: "https://cbgpu.com/feed",
    },
    KbEntry {
        id: "cbgpu-download",
        title: "앱 다운로드",
        content: "Android 앱 설치 파일과 설치 안내를 확인할 수 있습니다.",
        category: "app",
        keywords: &["앱", "다운로드", "Android", "설치", "모바일"],
        source: "https://cbgpu.com/download",
    },
    KbEntry {
        id: "cbgpu-contact",
        title: "문의하기",
        content: "문제와 제안을 보낼 수 있습니다. 사용 가이드, 문의, 개인정보처리방침 링크가 페이지 하단에 있습니다.",
        category: "support",
        keywords: &["문의", "지원", "도움", "연락", "제안", "버그"],
        source: "https://cbgpu.com/contact",
    },
];

pub fn search_kb(query: &str, limit: usize) -> Vec<&'static KbEntry> {
    let query_lower = query.to_lowercase();
    let mut scored: Vec<(&'static KbEntry, usize)> = KB_ENTRIES
        .iter()
        .filter_map(|entry| {
            let mut score = 0usize;
            let content_lower = entry.content.to_lowercase();
            let title_lower = entry.title.to_lowercase();
            for keyword in entry.keywords {
                let kw_lower = keyword.to_lowercase();
                if query_lower.contains(&kw_lower) {
                    score += 2;
                    if content_lower.contains(&kw_lower) || title_lower.contains(&kw_lower) {
                        score += 1;
                    }
                }
            }
            if score > 0 {
                Some((entry, score))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored
        .into_iter()
        .take(limit)
        .map(|(entry, _)| entry)
        .collect()
}

pub fn get_kb_entry(id: &str) -> Option<&'static KbEntry> {
    KB_ENTRIES.iter().find(|e| e.id == id)
}

pub fn list_kb_categories() -> Vec<&'static str> {
    let mut cats: Vec<&'static str> = KB_ENTRIES.iter().map(|e| e.category).collect();
    cats.sort();
    cats.dedup();
    cats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_kb_gpu() {
        let results = search_kb("GPU 예약", 5);
        assert!(!results.is_empty());
        assert_eq!(results[0].id, "cbgpu-gpu-computing");
    }

    #[test]
    fn test_search_kb_ai() {
        let results = search_kb("AI 대화", 5);
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.id == "cbgpu-ai-research-tools"));
    }

    #[test]
    fn test_search_kb_no_match() {
        let results = search_kb("zzzunknownquery", 5);
        assert!(results.is_empty());
    }

    #[test]
    fn test_get_kb_entry() {
        let entry = get_kb_entry("cbgpu-overview");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().title, "CBGPU 소개");
    }

    #[test]
    fn test_get_kb_entry_not_found() {
        assert!(get_kb_entry("nonexistent").is_none());
    }

    #[test]
    fn test_list_kb_categories() {
        let cats = list_kb_categories();
        assert!(!cats.is_empty());
        assert!(cats.contains(&"overview"));
    }

    #[test]
    fn test_kb_entries_verified() {
        for entry in KB_ENTRIES.iter() {
            assert!(!entry.id.is_empty());
            assert!(!entry.title.is_empty());
            assert!(!entry.content.is_empty());
            assert!(!entry.source.is_empty());
            assert!(!entry.keywords.is_empty());
        }
    }
}
