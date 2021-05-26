use chrono::{Duration, Utc};
use warp::{hyper::header, Reply};

#[derive(Debug, Clone)]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}
impl std::fmt::Display for SameSitePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SameSitePolicy::Strict => write!(f, "Strict"),
            SameSitePolicy::Lax => write!(f, "Lax"),
            SameSitePolicy::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CookieBuilder {
    domain: Option<String>,
    expires: Option<String>,
    max_age: Option<i64>,
    http_only: Option<()>,
    secure: Option<()>,
    path: Option<String>,
    same_site: Option<SameSitePolicy>,
    name: Option<String>,
    value: Option<String>,
}

impl CookieBuilder {
    pub fn new() -> Self {
        Self {
            domain: None,
            expires: None,
            max_age: None,
            http_only: None,
            secure: None,
            path: None,
            same_site: None,
            name: None,
            value: None,
        }
    }

    pub fn with_name(&self, name: String) -> Self {
        let mut new_builder = self.clone();
        new_builder.name = Some(name);
        new_builder
    }

    pub fn without_name(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.name = None;
        new_builder
    }

    pub fn with_value(&self, value: String) -> Self {
        let mut new_builder = self.clone();
        new_builder.value = Some(value);
        new_builder
    }

    pub fn without_value(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.value = None;
        new_builder
    }

    pub fn with_domain(&self, domain: String) -> Self {
        let mut new_builder = self.clone();
        new_builder.domain = Some(domain);
        new_builder
    }

    pub fn without_domain(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.domain = None;
        new_builder
    }

    pub fn with_expires(&self, expires: String) -> Self {
        let mut new_builder = self.clone();
        new_builder.expires = Some(expires);
        new_builder
    }

    pub fn without_expires(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.expires = None;
        new_builder
    }

    pub fn with_http_only(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.http_only = Some(());
        new_builder
    }

    pub fn without_http_only(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.http_only = None;
        new_builder
    }

    pub fn with_secure(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.secure = Some(());
        new_builder
    }

    pub fn without_secure(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.secure = None;
        new_builder
    }

    pub fn with_same_site(&self, same_site: SameSitePolicy) -> Self {
        let mut new_builder = self.clone();
        new_builder.same_site = Some(same_site);
        new_builder
    }

    pub fn without_same_site(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.same_site = None;
        new_builder
    }

    pub fn with_path(&self, path: String) -> Self {
        let mut new_builder = self.clone();
        new_builder.path = Some(path);
        new_builder
    }

    pub fn without_path(&self) -> Self {
        let mut new_builder = self.clone();
        new_builder.path = None;
        new_builder
    }

    pub fn cookie<T: Reply>(&self, reply: T) -> Result<warp::reply::WithHeader<T>, String> {
        if self.name.is_none() {
            return Err("No cookie name?".to_string());
        }
        let mut cookie_header;
        if self.value.is_some() {
            cookie_header = format!(
                "{}={}",
                self.name.clone().unwrap(),
                self.value.clone().unwrap()
            );
        } else {
            cookie_header = format!("{}=", self.name.clone().unwrap());
        }
        if self.expires.is_some() {
            cookie_header.push_str(format!("; Expires={}", self.expires.clone().unwrap()).as_str());
        }
        if self.max_age.is_some() {
            cookie_header.push_str(format!("; Max-Age={}", self.max_age.clone().unwrap()).as_str());
        }
        if self.domain.is_some() {
            cookie_header.push_str(format!("; Domain={}", self.domain.clone().unwrap()).as_str());
        }
        if self.path.is_some() {
            cookie_header.push_str(format!("; Path={}", self.path.clone().unwrap()).as_str());
        }
        if self.secure.is_some() {
            cookie_header.push_str(format!("; Secure").as_str());
        }
        if self.http_only.is_some() {
            cookie_header.push_str(format!("; HttpOnly").as_str());
        }
        if self.same_site.is_some() {
            cookie_header.push_str(
                format!("; SameSite={}", self.same_site.clone().unwrap().to_string()).as_str(),
            );
        }

        Ok(warp::reply::with_header(
            reply,
            header::SET_COOKIE,
            cookie_header,
        ))
    }

    pub fn cookie_with_value<T: Reply>(
        &self,
        reply: T,
        value: String,
    ) -> Result<warp::reply::WithHeader<T>, String> {
        if self.name.is_none() {
            return Err("No cookie name?".to_string());
        }
        let mut cookie_header;
        cookie_header = format!("{}={}", self.name.clone().unwrap(), value);
        if self.expires.is_some() {
            cookie_header.push_str(format!("; Expires={}", self.expires.clone().unwrap()).as_str());
        }
        if self.max_age.is_some() {
            cookie_header.push_str(format!("; Max-Age={}", self.max_age.clone().unwrap()).as_str());
        }
        if self.domain.is_some() {
            cookie_header.push_str(format!("; Domain={}", self.domain.clone().unwrap()).as_str());
        }
        if self.path.is_some() {
            cookie_header.push_str(format!("; Path={}", self.path.clone().unwrap()).as_str());
        }
        if self.secure.is_some() {
            cookie_header.push_str(format!("; Secure").as_str());
        }
        if self.http_only.is_some() {
            cookie_header.push_str(format!("; HttpOnly").as_str());
        }
        if self.same_site.is_some() {
            cookie_header.push_str(
                format!("; SameSite={}", self.same_site.clone().unwrap().to_string()).as_str(),
            );
        }

        Ok(warp::reply::with_header(
            reply,
            header::SET_COOKIE,
            cookie_header,
        ))
    }

    pub fn cookie_with_name<T: Reply>(
        &self,
        reply: T,
        name: String,
    ) -> Result<warp::reply::WithHeader<T>, String> {
        let mut cookie_header;
        if self.value.is_some() {
            cookie_header = format!("{}={}", name, self.value.clone().unwrap());
        } else {
            cookie_header = format!("{}=", name);
        }
        if self.expires.is_some() {
            cookie_header.push_str(format!("; Expires={}", self.expires.clone().unwrap()).as_str());
        }
        if self.max_age.is_some() {
            cookie_header.push_str(format!("; Max-Age={}", self.max_age.clone().unwrap()).as_str());
        }
        if self.domain.is_some() {
            cookie_header.push_str(format!("; Domain={}", self.domain.clone().unwrap()).as_str());
        }
        if self.path.is_some() {
            cookie_header.push_str(format!("; Path={}", self.path.clone().unwrap()).as_str());
        }
        if self.secure.is_some() {
            cookie_header.push_str(format!("; Secure").as_str());
        }
        if self.http_only.is_some() {
            cookie_header.push_str(format!("; HttpOnly").as_str());
        }
        if self.same_site.is_some() {
            cookie_header.push_str(
                format!("; SameSite={}", self.same_site.clone().unwrap().to_string()).as_str(),
            );
        }

        Ok(warp::reply::with_header(
            reply,
            header::SET_COOKIE,
            cookie_header,
        ))
    }

    pub fn cookie_with_value_and_name<T: Reply>(
        &self,
        reply: T,
        name: String,
        value: String,
    ) -> Result<warp::reply::WithHeader<T>, String> {
        if self.name.is_none() {
            return Err("No cookie name?".to_string());
        }
        let mut cookie_header;
        cookie_header = format!("{}={}", name, value);
        if self.expires.is_some() {
            cookie_header.push_str(format!("; Expires={}", self.expires.clone().unwrap()).as_str());
        }
        if self.max_age.is_some() {
            cookie_header.push_str(format!("; Max-Age={}", self.max_age.clone().unwrap()).as_str());
        }
        if self.domain.is_some() {
            cookie_header.push_str(format!("; Domain={}", self.domain.clone().unwrap()).as_str());
        }
        if self.path.is_some() {
            cookie_header.push_str(format!("; Path={}", self.path.clone().unwrap()).as_str());
        }
        if self.secure.is_some() {
            cookie_header.push_str(format!("; Secure").as_str());
        }
        if self.http_only.is_some() {
            cookie_header.push_str(format!("; HttpOnly").as_str());
        }
        if self.same_site.is_some() {
            cookie_header.push_str(
                format!("; SameSite={}", self.same_site.clone().unwrap().to_string()).as_str(),
            );
        }

        Ok(warp::reply::with_header(
            reply,
            header::SET_COOKIE,
            cookie_header,
        ))
    }

    pub fn cookie_with_value_and_expires_days<T: Reply>(
        &self,
        reply: T,
        value: String,
        expires: i64,
    ) -> Result<warp::reply::WithHeader<T>, String> {
        self.cookie_with_value_and_expires(
            reply,
            value,
            (Utc::now() + Duration::days(expires)).to_rfc2822(),
        )
    }

    pub fn cookie_with_value_and_expires<T: Reply>(
        &self,
        reply: T,
        value: String,
        expires: String,
    ) -> Result<warp::reply::WithHeader<T>, String> {
        if self.name.is_none() {
            return Err("No cookie name?".to_string());
        }
        let mut cookie_header;
        cookie_header = format!("{}={}", self.name.clone().unwrap(), value);
        cookie_header.push_str(format!("; Expires={}", expires).as_str());
        if self.max_age.is_some() {
            cookie_header.push_str(format!("; Max-Age={}", self.max_age.clone().unwrap()).as_str());
        }
        if self.domain.is_some() {
            cookie_header.push_str(format!("; Domain={}", self.domain.clone().unwrap()).as_str());
        }
        if self.path.is_some() {
            cookie_header.push_str(format!("; Path={}", self.path.clone().unwrap()).as_str());
        }
        if self.secure.is_some() {
            cookie_header.push_str(format!("; Secure").as_str());
        }
        if self.http_only.is_some() {
            cookie_header.push_str(format!("; HttpOnly").as_str());
        }
        if self.same_site.is_some() {
            cookie_header.push_str(
                format!("; SameSite={}", self.same_site.clone().unwrap().to_string()).as_str(),
            );
        }

        Ok(warp::reply::with_header(
            reply,
            header::SET_COOKIE,
            cookie_header,
        ))
    }
}
