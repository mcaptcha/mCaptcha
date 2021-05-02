/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::web::ServiceConfig;

pub mod auth;
pub mod mcaptcha;
pub mod meta;
pub mod pow;

pub fn services(cfg: &mut ServiceConfig) {
    // meta
    cfg.service(meta::build_details);
    cfg.service(meta::health);

    // auth
    cfg.service(auth::signout);
    cfg.service(auth::signin);
    cfg.service(auth::signup);
    cfg.service(auth::delete_account);
    cfg.service(auth::username_exists);
    cfg.service(auth::email_exists);
    cfg.service(auth::get_secret);
    cfg.service(auth::update_user_secret);
    cfg.service(auth::set_email);

    // mcaptcha
    cfg.service(mcaptcha::mcaptcha::add_mcaptcha);
    cfg.service(mcaptcha::mcaptcha::delete_mcaptcha);
    cfg.service(mcaptcha::mcaptcha::update_token);
    cfg.service(mcaptcha::mcaptcha::get_token);

    // levels
    cfg.service(mcaptcha::levels::add_levels);
    cfg.service(mcaptcha::levels::update_levels);
    cfg.service(mcaptcha::levels::delete_levels);
    cfg.service(mcaptcha::levels::get_levels);

    // duration
    cfg.service(mcaptcha::duration::update_duration);
    cfg.service(mcaptcha::duration::get_duration);
}

#[cfg(test)]
mod tests;