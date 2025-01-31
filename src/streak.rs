use kuchiki::NodeRef;
use kuchiki::traits::TendrilSink;

pub async fn fetch_streak_data(username: &str) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let svg_html = reqwest::get(format!("https://streak-stats.demolab.com/?user={}", username))
        .await
        .unwrap()
        .text().await.unwrap();

    let stats = {
        let document = kuchiki::parse_html().one(svg_html);

        let total_contributions = get_text_content(&document, "svg > g > g:nth-child(3) > g:nth-child(1) > text").unwrap_or("None".to_string());
        let current_streak = get_text_content(&document, "svg > g > g:nth-child(4) > g:nth-child(5) > text").unwrap_or("None".to_string());
        let longest_streak = get_text_content(&document, "svg > g > g:nth-child(5) > g:nth-child(1) > text").unwrap_or("None".to_string());

        (total_contributions, current_streak, longest_streak)
    };

    Ok(stats)
}

fn get_text_content(document: &NodeRef, selector: &str) -> Option<String> {
    document
        .select(selector)
        .ok()
        .and_then(|mut nodes| nodes.next())
        .and_then(|node| node.as_node().first_child())
        .and_then(|child| child.as_text()
            .map(|a| a.clone().into_inner())
            .or_else(|| Some("Not found".to_string()))
        )
}