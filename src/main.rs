use reqwest::header;
use patreonbot::config::Config;

const CAMPAIGN_API_URL: &'static str = "https://www.patreon.com/api/oauth2/v2/campaigns";
const POSTS_API_URL: &'static str = "https://www.patreon.com/api/oauth2/v2/posts";

#[derive(serde::Deserialize, Debug)]
struct CampaignIdsDataResp {
    id: String,
}

#[derive(serde::Deserialize, Debug)]
struct CampaignIdsResp {
    data: Vec<CampaignIdsDataResp>,
}

fn campaign_ids(cfg: &Config) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response: CampaignIdsResp = client
        .get(CAMPAIGN_API_URL)
        .header(header::AUTHORIZATION, format!("Bearer {}", cfg.access_token))
        .send()?
        .json::<CampaignIdsResp>()?;

    Ok(response.data.into_iter().map(|data| data.id).collect())
}

fn camp_id(cfg: &Config) -> Result<String, reqwest::Error> {
    let mut camp_ids = campaign_ids(&cfg).unwrap();
    if camp_ids.is_empty() {
        panic!("No campaign found!");
    } else if 1 < camp_ids.len() {
        panic!("More than one campaign found!");
    }

    Ok(camp_ids.remove(0))
}

#[derive(serde::Deserialize, Debug)]
struct PostsResp {
    data: Vec<PostsDataResp>,
    links: Option<PostsLinksResp>,
}

#[derive(serde::Deserialize, Debug)]
struct PostsDataResp {
    id: String,
}

#[derive(serde::Deserialize, Debug)]
struct PostsLinksResp {
    next: String
}

fn recent_posts(cfg: &Config, camp_id: &str, max_count: usize) -> Result<Vec<String>, reqwest::Error> {
    fn retrieve(cfg: &Config, url: &str, ids: &mut Vec<String>, cnt: usize) -> Result<Option<String>, reqwest::Error> {
        let client = reqwest::blocking::Client::new();

        let response: PostsResp = client
        .get(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", cfg.access_token))
        .send()?
        .json::<PostsResp>()?;
        
        let mut resp_ids = response.data.into_iter().map(|data| data.id).collect::<Vec<String>>();
        ids.append(&mut resp_ids);

        if cnt < ids.len() {
            ids.drain(0..(ids.len() - cnt));
        }
        Ok(response.links.map(|links| links.next))
    }
    
    let mut ids: Vec<String> = Vec::new();
    let mut url = format!("{}/{}/posts?page%5Bcount%5D=500", CAMPAIGN_API_URL, camp_id);
    while let Some(next) = retrieve(cfg, &url, &mut ids, max_count)? {
        url = next;
    }

    Ok(ids)
}

#[derive(serde::Deserialize, Debug)]
struct PostDetailResp {
    data: PostDetailDataResp,
}

#[derive(serde::Deserialize, Debug)]
struct PostDetailDataResp {
    attributes: PostDetailAttributeResp
    
}

#[derive(serde::Deserialize, Debug)]
struct PostDetailAttributeResp {
    title: String,
    url: String
}


fn post_detail(cfg: &Config, post_id: &str) -> Result<PostDetailResp, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    
    client
    .get(format!("{}/{}?fields%5Bpost%5D=title,url", POSTS_API_URL, post_id))
    .header(header::AUTHORIZATION, format!("Bearer {}", cfg.access_token))
    .send()?
    .json::<PostDetailResp>()
}

fn main() {
    let conf = Config::load();
    let campaign_id: String = camp_id(&conf).unwrap();
    let mut ids: Vec<String> = recent_posts(&conf, &campaign_id, 10).unwrap();

    if let Some(last_id) = ids.pop() {
        let detail: PostDetailResp = post_detail(&conf, &last_id).unwrap();
        println!("detail: {:?}", detail);
    }
}
