use crate::models::School;
use crate::state::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::{CreateEmbed, CreateEmbedFooter};

#[poise::command(slash_command, rename = "cbgpu_about")]
pub async fn cbgpu(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("CBGPU — 학교와 프로젝트를 이어가는 공간")
        .description(
            "학생의 일상부터 연구와 개발까지. 공개 프로젝트를 둘러보고, 내 계정으로 학교생활과 작업을 이어가세요.",
        )
        .url("https://cbgpu.com")
        .field(
            "📚 학교생활",
            "시간표, 급식, 커뮤니티를 확인하세요. 제공 기능은 학교마다 다릅니다.",
            false,
        )
        .field(
            "💻 개발과 프로젝트",
            "혼자 만든 작업도, 함께 만든 서비스도 계속 관리하세요.",
            false,
        )
        .field(
            "🤖 AI와 연구 도구",
            "AI에게 질문하고 연구에 필요한 자료와 아이디어를 정리하세요.",
            false,
        )
        .field(
            "🎮 GPU 컴퓨팅",
            "학교에 연결된 GPU를 예약하고 Jupyter로 작업하세요. 학교별 이용 권한이 필요합니다.",
            false,
        )
        .field(
            "📦 공개 프로젝트와 대회",
            "공개 프로젝트와 대회를 둘러보고 참여하세요.",
            false,
        )
        .field(
            "📖 사용 가이드",
            "CBGPU 기능별 사용법과 문제 해결 안내는 https://docs.cbgpu.com 에서 확인하세요.",
            false,
        )
        .footer(CreateEmbedFooter::new("더 많은 정보: https://cbgpu.com"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_features")]
pub async fn features(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("CBGPU 주요 기능")
        .description("CBGPU에서 제공하는 핵심 기능들을 소개합니다.")
        .url("https://cbgpu.com/services")
        .field(
            "📚 학교생활",
            "시간표, 급식, 커뮤니티를 확인할 수 있습니다. 제공 기능은 학교마다 다릅니다.",
            false,
        )
        .field(
            "💻 개발과 프로젝트",
            "혼자 만든 작업도, 함께 만든 서비스도 계속 관리할 수 있습니다.",
            false,
        )
        .field(
            "🤖 AI와 연구 도구",
            "AI에게 질문하고 연구에 필요한 자료와 아이디어를 정리할 수 있습니다. AI 대화, 자료 검색, 글·코드 작업을 지원합니다.",
            false,
        )
        .field(
            "🎮 GPU 컴퓨팅",
            "학교에 연결된 GPU를 예약하고 Jupyter로 작업할 수 있습니다. 학교별 이용 권한이 필요합니다.",
            false,
        )
        .field(
            "📦 공개 프로젝트와 대회",
            "직접 공개하기로 선택한 프로젝트와 대회를 누구나 볼 수 있습니다. 행사 개설, 참가, 운영, 작품 제출까지 지원합니다.",
            false,
        )
        .field(
            "📖 사용 가이드",
            "CBGPU 기능별 사용법과 문제 해결 안내는 https://docs.cbgpu.com (51개 가이드)에서 확인할 수 있습니다.",
            false,
        )
        .footer(CreateEmbedFooter::new("자세한 정보: https://cbgpu.com"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_participating_schools")]
pub async fn schools(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;
    let schools: Vec<School> = storage
        .storage()
        .schools
        .iter()
        .filter(|s| s.cbgu_participating)
        .cloned()
        .collect();
    drop(storage);

    if schools.is_empty() {
        let storage = ctx.data().storage.lock().await;
        let all_schools: Vec<School> = storage.storage().schools.clone();
        drop(storage);

        if all_schools.is_empty() {
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("등록된 학교가 없습니다."),
            )
            .await?;
            return Ok(());
        }

        let mut description =
            String::from("아직 CBGPU에 참여 중인 학교가 없습니다.\n\n등록된 학교 목록:\n");
        for s in all_schools.iter() {
            description.push_str(&format!("• {} (참여 미확인)\n", s.name));
        }

        let embed = CreateEmbed::new()
            .title("참여 학교 — CBGPU")
            .description(crate::util::truncate(&description, crate::util::EMBED_DESC_MAX));
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let mut description = String::new();
    for s in schools.iter() {
        description.push_str(&format!("• {} — {}명의 대사\n", s.name, s.ambassador_count));
    }

    let embed = CreateEmbed::new()
        .title("참여 학교 — CBGPU")
        .description(crate::util::truncate(&description, crate::util::EMBED_DESC_MAX));
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_links")]
pub async fn links(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::new()
        .title("CBGPU 주요 링크")
        .description("CBGPU 관련 주요 사이트와 자료 모음입니다.")
        .url("https://cbgpu.com")
        .field(
            "🏠 CBGPU 홈",
            "https://cbgpu.com — 학교와 프로젝트를 이어가는 공간",
            false,
        )
        .field(
            "📖 사용 가이드",
            "https://docs.cbgpu.com — 기능별 사용법과 문제 해결 안내 (51개 가이드)",
            false,
        )
        .field(
            "🔬 연구·개발 공간",
            "https://researcher.cbgpu.com — GPU, 배포, 논문, 제작을 위한 연구 환경",
            false,
        )
        .field(
            "📄 연구 프리프린트",
            "https://arxiv.cbgpu.com — 연구 논문 검색 및 게시",
            false,
        )
        .field(
            "📱 앱 다운로드",
            "https://cbgpu.com/download — Android 앱 설치 파일과 설치 안내",
            false,
        )
        .field(
            "📬 문의하기",
            "https://cbgpu.com/contact — 문제와 제안 보내기",
            false,
        )
        .field(
            "📦 공개 프로젝트",
            "https://cbgpu.com/projects — 공개된 프로젝트 둘러보기",
            false,
        )
        .field(
            "🏆 공개 대회",
            "https://cbgpu.com/explore/contests — 공개된 대회와 행사",
            false,
        )
        .footer(CreateEmbedFooter::new("더 많은 정보: https://cbgpu.com"));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_school_participation")]
pub async fn school_participation(
    ctx: Context<'_>,
    #[description = "School name"] school: String,
    #[description = "Is the school participating in CBGPU?"] participating: bool,
) -> Result<(), Error> {
    if !crate::commands::mission::admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can change school participation."),
        )
        .await?;
        return Ok(());
    }
    let actor = ctx.author().id.get();
    let mut storage = ctx.data().storage.lock().await;
    let idx = match storage.find_school_by_name(&school) {
        Some(i) => i,
        None => {
            storage.storage_mut().schools.push(School::new(school.clone()));
            storage.storage().schools.len() - 1
        }
    };
    storage.storage_mut().schools[idx].cbgu_participating = participating;
    let name = storage.storage().schools[idx].name.clone();
    storage.audit_log(
        actor,
        "school_participation".to_string(),
        name.clone(),
        format!("participating = {participating}"),
    );
    storage.save()?;
    drop(storage);
    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(format!("{name}: participating = {participating}")),
    )
    .await?;
    Ok(())
}
